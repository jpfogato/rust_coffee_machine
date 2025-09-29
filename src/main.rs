pub mod controller;

fn main() {

    const ESPRESSO:controller::Recipe = controller::Recipe{
                water_amount_ml: 50,
                ground_coffee_amount_g: 45,
                brew_time_s: 35,
                coarseness: controller::Coarseness::Medium,
                double: false
    };

    const LUNGO:controller::Recipe = controller::Recipe {
                water_amount_ml: 100,
                ground_coffee_amount_g: 25,
                brew_time_s: 40,
                coarseness: controller::Coarseness::Fine,
                double: false,
            };

    const AMERICANO:controller::Recipe = controller::Recipe {
                water_amount_ml: 150,
                ground_coffee_amount_g: 10,
                brew_time_s: 0,
                coarseness: controller::Coarseness::Coarse,
                double: false,
            };

    let mut resources = controller::Resources{
        amount_of_water_ml: 0,
        amount_of_coffee_beans_g: 0,
        amount_of_residues_g: 0,
    };

    controller::sim_user_fill_water(45, &mut resources);
    controller::sim_user_fill_water(45, &mut resources);

}
