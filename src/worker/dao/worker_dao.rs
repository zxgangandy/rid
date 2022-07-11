
use rbatis::crud::{CRUD};
use rbatis::rbatis::Rbatis;
use super::RB;
use crate::model::miner::Miners;
use crate::worker::model::worker_node;
use rbatis::Error;

pub struct WorkerDao {
    RB: Rbatis,
}

impl WorkerDao {
    pub fn new(RB: Rbatis) -> Self {
        WorkerDao {RB}
    }

    pub async fn save(&self, w: worker_node::WorkerNode) -> Result<bool, Error> {
        let save_res = self.RB.save(&w, &[]).await;
        if let Ok(res) = save_res {
            let affect_row = res.rows_affected;
            if affect_row > 0 {
                return Ok(true);
            }

            return Ok(false);
        }

        Err(item_res.unwrap_err())
    }

    pub async fn get_by_hostname(&self, host_name: &String) -> Result<worker_node::WorkerNode, Error> {
        let w = self.RB.new_wrapper().eq("host_name", host_name);
        let item_res = self.RB.fetch_by_wrapper::<worker_node::WorkerNode>(w).await;
        if let Ok(res) = item_res {
            return Ok(res);
        }

        Err(item_res.unwrap_err())
    }
}