/* Tests module with all of the API testings for the application
*/

#[cfg(test)]
mod test {

    // use crate::controller::CoffeeMachine;
    use crate::file_operations::file_handler::{FileHandler, FileOffsets};
    use core::panic;
    use std::{
        collections::btree_map::IterMut,
        fs::{self, File},
        vec,
    };

    // support functions
    fn get_vector_from_file(path: &str) -> Vec<u8> {
        // goes directly to the file in the disk and fetchs its whole value
        let mut file_buffer: Vec<u8> = vec![0; 0];
        match fs::read(path).ok() {
            Some(i) => {
                file_buffer = i;
            }
            None => panic!("could not retrieve runtime values from file"),
        }
        file_buffer
    }

    fn instantiate_file_handler() -> FileHandler {
        // returns a new FileHandler struct to be used in downstream testing
        FileHandler::new()
    }

    fn test_file_content<'a>(
        // tests the file from disk directly against what's in the class
        context: &'a FileHandler,
        file_contents: &'a [u8],
        item_to_test: &FileOffsets,
    ) -> &'a [u8] {
        let buffer_item = context.file_offsets.get(&item_to_test);
        if let Some(tuple) = buffer_item {
            let index: usize = tuple.0 as usize;
            let length: usize = tuple.1 as usize + index;
            &file_contents[index..length]
        } else {
            panic!("could not verify file contents")
        }
    }

    // FileHandler tests
    #[test] // all tests must implement this attribute
    fn test_all_structs() {
        let file_handler = instantiate_file_handler();
        let file_contents = get_vector_from_file("/tmp/coffee_machine/runtime.bin");

        // add file positions here for testing
        let item_to_test = [
            FileOffsets::Endianess,
            FileOffsets::NumberOfCoffessBrewed,
            FileOffsets::NeedsCoffeeBeans,
            FileOffsets::NeedsWater,
            FileOffsets::NeedsGroundsRemoval,
            FileOffsets::NeedsDescaling,
            FileOffsets::DefaultWaterDosage,
            FileOffsets::DefaultCoffeeDosage,
        ];

        for offset in item_to_test {
            let file_slice = test_file_content(&file_handler, &file_contents, &offset);
            println!("testing {:?} -> {:?}", &offset, &file_slice);

            assert_eq!(&file_handler.get_param(offset), &file_slice);
        }
    }

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
