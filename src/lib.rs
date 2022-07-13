pub mod rid_generator;
pub mod bits_allocator;
pub mod worker;
pub mod config;

#[macro_use]
extern crate rbatis;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
