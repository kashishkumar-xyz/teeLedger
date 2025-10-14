pub mod models;
pub mod db;
pub mod ffi;
pub mod key_management;
pub mod backup;
pub mod recovery;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // Placeholder for lib.rs level tests if any
        assert!(true);
    }
}
