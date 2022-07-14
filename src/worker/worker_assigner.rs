use std::sync::Arc;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use std::env;
use rbatis::rbatis::Rbatis;

use crate::worker::service::worker_service;
use crate::worker::model::worker_node;

const CONTAINER: i8               = 1; // container type
const ACTUAL: i8                  = 2; // actual type

const ENV_KEY_HOST: &str          = "JPAAS_HOST";           // env key host
const ENV_KEY_PORT: &str          = "JPAAS_HTTP_PORT";      // env key port
const ENV_KEY_PORT_ORIGINAL: &str = "JPAAS_HOST_PORT_8080"; // env key 8080 port
const COMPUTER_NAME: &str         = "COMPUTERNAME";         //computer name

pub struct Assigner {
    worker_node_service: worker_service::WorkerService,
    port: String,
}

impl Assigner {
    // new_worker_id_assigner create worker id assigner instance
    pub fn new(port: String, RB: Arc<Rbatis>) -> Self {
        let worker_node_service = worker_service::WorkerService::new(Arc::clone(&RB));
        Assigner { worker_node_service, port }
    }

    pub async fn assign_worker_id(&self) -> i64 {
        let new_node = Assigner::build_worker_node(&self.port);
        let node = self.worker_node_service.get_by_hostname(
            &new_node.clone().host_name.unwrap()
        ).await;

        if let Ok(res) = node {
            return res.id.unwrap() as i64;
        }

        let _= self.worker_node_service.save(new_node.clone()).await;
        return new_node.id.unwrap() as i64;
    }

    fn build_worker_node(port: &String) -> worker_node::WorkerNode {
        let now = NaiveDateTime::now().into();
        let mut node = worker_node::WorkerNode {
            id: None,
            host_name: None,
            port: Some(port.clone()),
            worker_type: None,
            launch_date: now,
            modified: now,
            created: now
        };

        let host_var = env::var(ENV_KEY_HOST);
        if let Ok(res) = host_var {
            node.host_name = Some(res);
            node.worker_type = Some(CONTAINER);
            node.port = Assigner::get_docker_port();
        } else {
            node.worker_type = Some(ACTUAL);
            node.host_name = Assigner::get_computer_hostname();
            node.port = Some(port.clone());
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

