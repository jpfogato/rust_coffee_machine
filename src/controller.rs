/*
Implements these classes:
    - CoffeeMachine: implements all methods related to the Coffee Machine operation and business logic
    - Recipe: current recipe selected by the user via the ui
    - Resources: tracks water, beans and waste disposal levels

TODOs
1. implement file reading so values from a previous operation are store as a byte array and parsed to initialize the class

*/

pub struct CoffeeMachine {
    // these values should not be reseted after shutdown
    global_coffees_made: u32,
    file_location: String,
}

impl CoffeeMachine {
    fn init() -> (Self, RecipeInterface, Resources) {
        /*
            Initializes the coffee machine by returning an instance of Self, Recipe and Resources for operations
        */
        return (
            Self {
                global_coffees_made: 0,
                file_location: String::from("To do..."),
            },
            RecipeInterface::init(),
            Resources::init(),
        );
    }

    pub fn grind(&mut self, resources: &mut Resources, recipe: &mut RecipeInterface) {
        if resources.check_resources() {
            println!("grinding coffee...");

            resources.increment_ground_coffee_counter();

            self.global_coffees_made += 1;

            resources.amount_of_coffee_beans_g -= recipe.ground_coffee_amount_g;
            resources.amount_of_water_ml -= recipe.water_amount_ml;
        } else {
            println!("Not enough resources to do a coffee");
        }
    }

    pub fn heat_water() {}

    pub fn add_hot_water() {}

    pub fn brew() {}
}

#[derive(Debug)]
pub enum Coarseness {
    Fine,
    Medium,
    Coarse,
}
#[derive(Debug)]
pub struct RecipeInterface {
    water_amount_ml: u32, // default values: short: 50ml, long: 100ml
    ground_coffee_amount_g: u32,
    coffee_dosage_g: u32, // varies from 10 to 50 in grams
    coarseness: Coarseness,
    double: bool,
}

impl RecipeInterface {
    // Public methods

    fn init() -> Self {
        Self {
            water_amount_ml: 0,
            ground_coffee_amount_g: 0,
            double: false,
            coarseness: Coarseness::Medium,
            coffee_dosage_g: 25,
        }
    }

    pub fn update_coarseness(&mut self, level: &str) {
        // Return a Coarseness type depending on the user's input
        self.coarseness = match level.trim().parse() {
            Ok(1) => Coarseness::Coarse,
            Ok(2) => Coarseness::Medium,
            Ok(3) => Coarseness::Fine,
            Ok(_) => {
                println!("Invalid value. Coerced to \"Medium\"");
                Coarseness::Medium
            }
            Err(_) => {
                println!("Could not resolve from input. Coerced to \"Medium\"");
                Coarseness::Medium
            }
        };

        self.update_coffee_dosage_g();
        // update dosage afterwards
    }

    // Private methods

    fn update_coffee_dosage_g(&mut self) {
        // depending on the coarseness and if a double is selected or not updates the coffee dosage
        self.coffee_dosage_g = match self.coarseness {
            Coarseness::Coarse => 10,
            Coarseness::Medium => 25,
            Coarseness::Fine => 50,
        };
        if self.double {
            // if a double cofee is selected, increase dosage by 50%
            self.coffee_dosage_g *= 1.5 as u32
        }
    }
}

pub struct Resources {
    amount_of_water_ml: u32,
    amount_of_coffee_beans_g: u32,
    amount_of_residues_g: u32,
    current_coffees_ground: u32,
    needs_water: bool,
    needs_beans: bool,
    needs_grounds_removal: bool,
}

impl Resources {
    // Public methods

    pub fn init() -> Self {
        Self {
            amount_of_coffee_beans_g: 0,
            amount_of_residues_g: 0,
            amount_of_water_ml: 0,
            current_coffees_ground: 0,
            needs_water: false,
            needs_beans: false,
            needs_grounds_removal: false,
        }
    }
    pub fn sim_user_add_water(&mut self, amount_added_ml: u32) {
        // updates Resources in-place
        self.amount_of_water_ml = self.amount_of_water_ml + amount_added_ml
    }

    pub fn sim_user_add_beans(&mut self, amount_added_g: u32) {
        // updates Resources in-place
        self.amount_of_coffee_beans_g += self.amount_of_coffee_beans_g + amount_added_g
    }

    pub fn count_residues(&mut self, recipe: &RecipeInterface) {
        // increment the amount of residues in the coffee machine
        self.amount_of_residues_g += recipe.ground_coffee_amount_g
    }

    pub fn check_resources(&mut self) -> bool {
        // returns true if less than 50g of beans, or
        // less than 100ml of water, or
        // more than 1000g of residues

        // TODO return which resource is missing by altering reference in place
        self.amount_of_coffee_beans_g < 50
            || self.amount_of_water_ml < 100
            || self.amount_of_residues_g >= 1000
    }

    fn increment_ground_coffee_counter(&mut self) {
        self.current_coffees_ground += 1;
    }
}
