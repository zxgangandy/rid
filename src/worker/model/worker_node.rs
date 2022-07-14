use chrono::{DateTime, NaiveDateTime};
use chrono::Date;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:worker_node)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkerNode{
    pub id: Option<u64>,
    pub host_name: Option<String>,
    pub port: Option<String>,
    pub worker_type: Option<i8>,
    pub modified: Option<NaiveDateTime>,
    pub created: Option<NaiveDateTime>,
}
