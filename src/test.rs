/* Tests module with all of the API testings for the application
*/

#[cfg(test)]
mod api_testing {
    // use crate::controller::CoffeeMachine;
    use crate::file_operations::file_handler::FileHandler;
    use crate::file_operations::file_handler::FileOffsets;

    // FileHandler tests
    #[test] // all tests must implement this attribute
    fn test_initialization() {
        let file_handler = FileHandler::new();
        assert_eq!(file_handler, file_handler);
    }

    #[test]
    fn test_update_buffer() {
        let mut file_handler = FileHandler::new();
        let data: u64 = 500;
        file_handler.update_params_on_buffer(data, FileOffsets::DefaultCoffeeDosage);
        assert_eq!(
            file_handler.get_value_from_buffer(FileOffsets::DefaultCoffeeDosage),
            1
        );
    }
}
