/* Tests module with all of the API testings for the application
*/

#[cfg(test)]
mod api_testing {
    // use crate::controller::CoffeeMachine;
    use crate::file_operations::file_handler::FileHandler;

    // FileHandler tests
    #[test] // all tests must implement this attribute
    fn test_initialization() {
        let file_handler = FileHandler::new();
        assert_eq!(file_handler, file_handler);
    }

    #[test]
    fn test_update_buffer() {}
}

mod controller_testing {

    // controller tests
    #[test]
    fn controller_tester() {
        // initializing the machine, the recipe and the resources
        let (mut coffe_machine, mut recipe, mut resources) =
            crate::controller::CoffeeMachine::init();

        // adding 500g of beans and 1000ml of water
        resources.sim_user_add_beans(500);
        resources.sim_user_add_water(1000);
        //dbg!(&resources);

        // setting the recipe we want
        // max coffee dosage between 10 and 20g,
        recipe.set_coffee_dosage(15.to_string());
        recipe.set_double(false);
        recipe.update_recipe();

        // grinding and brewing a coffee...
        coffe_machine.grind(&mut resources, &mut recipe);
        coffe_machine.brew(&mut resources, &recipe);

        // checking our resources left
        let resource_snapshot = resources.get_resource_amount();
        println!(
            "Resources left:\n\t{}g of beans\n\t{}ml of water",
            resource_snapshot.0, resource_snapshot.1
        );
    }
}
