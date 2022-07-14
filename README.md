# rid
A pratical distributed id generator by rust

[Snowflake](https://github.com/twitter/snowflake) based unique ID generator. It
works as a component, and allows users to override workId bits and initialization strategy. As a result, it is much more
suitable for virtualization environment, such as [docker](https://www.docker.com/).

## Snowflake

**Snowflake algorithm：**
An unique id consists of worker node, timestamp and sequence within that timestamp. Usually, it is a 64 bits number(long), and the default bits of that three fields are as follows:
```xml
+------+----------------------+----------------+-----------+
| sign |     delta seconds    | worker node id | sequence  |
+------+----------------------+----------------+-----------+
  1bit          30bits              20bits         13bits
```

sign(1bit)
The highest bit is always 0.

delta seconds (30 bits)
The next 30 bits, represents delta seconds since a customer epoch(2016-05-20). The maximum time will be 34 years.

worker id (20 bits)
The next 20 bits, represents the worker node id, maximum value will be 1.04 million. UidGenerator uses a build-in database based worker id assigner when startup by default, and it will reuse previous work node id after reboot.

sequence (13 bits)
the last 13 bits, represents sequence within the one second, maximum is 8192 per second（per server）by default.

## Features
- light and easy to use
- distributed id generator at local instead of by service or rpc
- worker id persistence solution (in database like mysql instead of cache storage)
- support clock moved backwards(can be disabled)
- support id length customization lower than 64 bits


## Design
- refer to baidu [uid-generator](https://github.com/baidu/uid-generator)


## Quick  Start

### Step1: Install rust, Mysql

### Step2: Create table worker_node

```sql
DROP TABLE IF EXISTS `worker_node`;
CREATE TABLE `worker_node` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'auto increment id',
  `host_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'host name',
  `port` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'port',
  `type` int NOT NULL COMMENT 'node type: CONTAINER(1), ACTUAL(2), FAKE(3)',
  `modified` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'modified time',
  `created` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'created time',
  PRIMARY KEY (`id`),
  UNIQUE KEY `UNIQ_IDX_HOST_PORT` (`host_name`,`port`) USING BTREE COMMENT 'host和端口的唯一索引'
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DB WorkerID Assigner for UID Generator';

```

### Step3: Install Lib


### Step4: Usage

```rust
let config = rid_config::UidConfig::new("5000".to_string());
let rb: Rbatis = Rbatis::new();
rb.link("mysql://root:root@127.0.0.1:3306/test")
.await
.expect("Couldn't open database");
let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(rb)).await;

let start = Local::now().timestamp_millis();
for _ in 1..10000 {
    //println!("{}", &idg.get_uid());
    let _ = &idg.get_uid();
}

```

## Customization

Change the time_bits, worker_bits, seq_bits of 'UidConfig' to get your customer uid, especially shorter uid.

```rust
let mut config = rid_config::UidConfig::new("5000".to_string());
config.worker_bits = 10;
config.seq_bits = 23;

let rb: Rbatis = Rbatis::new();
rb.link("mysql://root:root@127.0.0.1:3306/test")
.await
.expect("Couldn't open database");
let mut idg = rid_generator::UidGenerator::new(&config, Arc::new(rb)).await;

let start = Local::now().timestamp_millis();
for _ in 1..1000000 {
    //println!("{}", &idg.get_uid());
    let _ = &idg.get_uid();
}

```

## ChangeLog


## License
Gid is [MIT licensed](./LICENSE).