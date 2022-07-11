use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use std::env;

use crate::worker::service::worker_service;
use crate::config::rid_config;
use crate::worker::model::worker_node;
const CONTAINER: i8 = 1; // container type
const ACTUAL: i8   = 2; // actual type

const ENV_KEY_HOST: &str          = "JPAAS_HOST";           // env key host
const ENV_KEY_PORT: &str          = "JPAAS_HTTP_PORT";      // env key port
const ENV_KEY_PORT_ORIGINAL: &str = "JPAAS_HOST_PORT_8080"; // env key 8080 port
const COMPUTER_NAME: &str         = "COMPUTERNAME";         //computer name

 struct DisposableWorkerIdAssigner {
    worker_node_service: worker_service::WorkerService,
    config: dyn rid_config::UidConfig,
}

// new_worker_id_assigner create worker id assigner instance
fn new_worker_id_assigner(config: &impl rid_config::UidConfig) -> &DisposableWorkerIdAssigner {
    let worker_node_service = worker_service::WorkerService::new(config.GetDB());
    return & DisposableWorkerIdAssigner{
        worker_node_service,
        config
    }
}

impl DisposableWorkerIdAssigner {
    pub async fn assign_worker_id(&self) -> u64 {
        let new_node = DisposableWorkerIdAssigner::build_worker_node(self.config.GetPort());
        let node = self.worker_node_service.get_by_hostname(&new_node.host_name.unwrap()).await;
        if let Ok(res) = node {
            return res.id.unwrap();
        }
        let _= self.worker_node_service.save(new_node).await;

        return new_node.id.unwrap();
    }

    fn build_worker_node(port: string) -> worker_node::WorkerNode {
        let now = NaiveDateTime::now().into();
        let mut node = worker_node::WorkerNode {
            id: None,
            host_name: None,
            port: Some(port),
            worker_type: None,
            launch_date: now,
            modified: now,
            created: now
        };

        let host_var = env::var(ENV_KEY_HOST);
        if let Ok(res) = host_var {
            node.host_name = Some(res);
            node.worker_type = Some(CONTAINER);
        } else {
            node.worker_type = Some(ACTUAL);
            node.host_name = Some(res);
        }

        if config.IsDocker {
            node.host_name = Some(env::var(ENV_KEY_HOST).unwrap());
            node.Port = DisposableWorkerIdAssigner::get_docker_port();
            node.worker_type = Some(CONTAINER);
        } else {
            node.worker_type = Some(ACTUAL);
            node.host_name = DisposableWorkerIdAssigner::get_computer_hostname();
            node.Port = port;
        }

        return node
    }

    fn get_docker_port()-> Option<String> {
        let port = env::var(ENV_KEY_PORT);
        if let Ok(res) = port {
            return Some(res)
        }

        env::var(ENV_KEY_PORT_ORIGINAL).ok()
    }

    fn get_computer_hostname() -> Option<String> {
        let hostname = env::var(COMPUTER_NAME);
        if let Ok(res) = hostname {
            return Some(res)
        }

        let result = hostname::get().unwrap();
        return result.into_string().ok();
    }
}

