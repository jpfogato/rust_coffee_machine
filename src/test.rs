/* Tests module with all of the API testings for the application
*/

#[cfg(test)]
mod api_testing {
    // use crate::controller::CoffeeMachine;
    use crate::file_operations::file_handler::FileHandler;

    // FileHandler tests
    #[test] // all tests must implement this attribute
    fn setup_file_handler() {
        let file_handler = FileHandler::new();
        assert_eq!(file_handler, file_handler)
    }
}
