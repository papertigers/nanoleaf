pub mod client;
pub mod error;
mod http_client;
pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
