pub mod rid_generator;
pub mod bits_allocator;
pub mod worker;
pub mod config;

#[macro_use]
extern crate rbatis;

#[cfg(test)]
mod tests {
    use crate::rid_generator;
    use crate::config::rid_config;
    use std::sync::Arc;
    use rbatis::rbatis::Rbatis;
    use async_std;
    use chrono::Local;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn default_config_test() {
        async_std::task::block_on(async {
            let mut config = rid_config::UidConfig::new("5000".to_string());

            let RB: Rbatis = Rbatis::new();
            RB.link("mysql://root:root@127.0.0.1:3306/test")
                .await
                .expect("Couldn't open database");
            let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(RB)).await;

            let start = Local::now().timestamp_millis();
            for _ in 1..10000 {
                //println!("{}", &idg.get_uid());
                let a = &idg.get_uid();
            }

            let end = Local::now().timestamp_millis();

            println!("{}", end-start);
        });

    }

    #[test]
    fn custom_config_test() {
        async_std::task::block_on(async {
            let mut config = rid_config::UidConfig::new("5000".to_string());
            config.worker_bits = 10;
            config.seq_bits = 23;

            let RB: Rbatis = Rbatis::new();
            RB.link("mysql://root:root@127.0.0.1:3306/test")
                .await
                .expect("Couldn't open database");
            let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(RB)).await;

            let start = Local::now().timestamp_millis();
            for _ in 1..1000000 {
                //println!("{}", &idg.get_uid());
                let a = &idg.get_uid();
            }

            let end = Local::now().timestamp_millis();

            println!("{}", end-start);
        });

    }
}
