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
