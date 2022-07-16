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
    fn gid_with_default_config() {
        async_std::task::block_on(async {
            let config = rid_config::UidConfig::new("5000".to_string());
            let rb: Rbatis = Rbatis::new();
            rb.link("mysql://root:root@127.0.0.1:3306/test")
                .await
                .expect("Couldn't open database");
            let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(rb)).await;

            let start = Local::now().timestamp_millis();
            for _ in 0..10000 {
                //println!("{}", &idg.get_uid());
                let _ = idg.get_uid();
            }

            let end = Local::now().timestamp_millis();
            println!("{}", end-start);
        });
    }

    #[test]
    fn pid_with_default_config() {
        async_std::task::block_on(async {
            let config = rid_config::UidConfig::new("5000".to_string());
            let rb: Rbatis = Rbatis::new();
            rb.link("mysql://root:root@127.0.0.1:3306/test")
                .await
                .expect("Couldn't open database");
            let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(rb)).await;

            let rid = idg.get_uid();
            println!("{}", rid);
            let pid = idg.parse_uid(rid);
            println!("{}", pid);
        });
    }

    #[test]
    fn pid_with_custom_config() {
        async_std::task::block_on(async {
            let mut config = rid_config::UidConfig::new("5000".to_string());
            config.worker_bits = 10;
            config.seq_bits = 23;
            let rb: Rbatis = Rbatis::new();
            rb.link("mysql://root:root@127.0.0.1:3306/test")
                .await
                .expect("Couldn't open database");
            let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(rb)).await;

            let rid = idg.get_uid();
            println!("{}", rid);
            let pid = idg.parse_uid(rid);
            println!("{}", pid);
        });
    }

    #[test]
    fn gid_with_custom_config() {
        async_std::task::block_on(async {
            let mut config = rid_config::UidConfig::new("5000".to_string());
            config.worker_bits = 10;
            config.seq_bits = 23;

            let rb: Rbatis = Rbatis::new();
            rb.link("mysql://root:root@127.0.0.1:3306/test")
                .await
                .expect("Couldn't open database");
            let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(rb)).await;

            let start = Local::now().timestamp_millis();
            for _ in 0..10000000 {
                //println!("{}", &idg.get_uid());
                let _ = idg.get_uid();
            }

            let end = Local::now().timestamp_millis();
            println!("{}", end-start);
        });
    }

}
