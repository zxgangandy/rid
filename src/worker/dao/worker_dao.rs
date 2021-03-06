use rbatis::crud::{CRUD};
use rbatis::rbatis::Rbatis;

use crate::worker::model::worker_node;
use rbatis::Error;

pub struct WorkerDao {
    rb: & 'static Rbatis,
}

impl WorkerDao {
    pub fn new(rb: & 'static Rbatis) -> Self {
        WorkerDao {
            rb
        }
    }

    pub async fn save(&self, w: worker_node::WorkerNode) -> Result<i64, Error> {
        let save_res = self.rb.save(&w, &[]).await;
        if let Ok(res) = save_res {
            let affect_row = res.rows_affected;
            if affect_row > 0 {
                return Ok(res.last_insert_id.unwrap());
            }

            return Ok(0);
        }

        Err(save_res.unwrap_err())
    }

    pub async fn get_by_hostname(&self, host_name: &String) -> Result<worker_node::WorkerNode, Error> {
        let w = self.rb.new_wrapper().eq("host_name", host_name);
        let item_res = self.rb.fetch_by_wrapper::<worker_node::WorkerNode>(w).await;
        if let Ok(res) = item_res {
            return Ok(res);
        }

        Err(item_res.unwrap_err())
    }
}