use crate::controller::{CoffeeMachine, RecipeInterface, Resources};

pub mod controller;

fn main() {

    let mut recipe = RecipeInterface::init();
    let mut machine = CoffeeMachine::init();
    let mut resources = Resources::init();

    resources.sim_user_add_beans(500);
    resources.sim_user_add_water(1000);

    recipe.set_coffee_dosage(String::from("8"));

    recipe.set_double(false);

    recipe.update_recipe();

}
