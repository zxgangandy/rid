use rbatis::Error;
use rbatis::rbatis::Rbatis;

use crate::worker::dao::worker_dao;
use crate::worker::model::worker_node;

pub struct WorkerService {
    worker_dao: worker_dao::WorkerDao,
}

impl WorkerService {
    pub fn new(rb: &'static Rbatis) -> Self {
        Self { worker_dao: worker_dao::WorkerDao::new(rb)  }
    }

    pub async fn save(&self, w: worker_node::WorkerNode) -> Result<i64, Error> {
        return self.worker_dao.save(w).await;
    }

    pub async fn get_by_hostname(&self, host_name: &String) -> Result<worker_node::WorkerNode, Error> {
        return self.worker_dao.get_by_hostname(host_name).await;
    }

}