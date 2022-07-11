pub mod worker_dao;


// use lazy_static::lazy_static;
// use rbatis::rbatis::Rbatis;
//
// lazy_static! {
//     pub static ref RB: Rbatis=Rbatis::new();
// }
//
// pub async fn open() {
//     RB.link("mysql://root:root@127.0.0.1:3306/test")
//         .await
//         .expect("Couldn't open database");
// }