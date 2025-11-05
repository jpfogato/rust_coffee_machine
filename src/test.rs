/* Tests module with all of the API testings for the application
*/

use crate::controller::CoffeeMachine;
use crate::file_operations::FileHandler;

#[cfg(test)]
mod test {
    use crate::file_operations::FileHandler;

    // CoffeeMachine tests
    #[test] // all tests must implement this attribute
    fn name() {
        todo!();
    }

    // FileHandler tests
    #[test]
    fn setup_file_handler() {
        let mut file_handler = FileHandler::initialize_file_handler();
    }
}
