/*
Implements these classes:
    - CoffeeMachine: implements all methods related to the Coffee Machine operation and business logic
    - Recipe: current recipe selected by the user via the ui
    - Resources: tracks water, beans and waste disposal levels

TODOs
1. implement file reading so values from a previous operation are store as a byte array and parsed to initialize the class

*/

#[derive(Debug)]
pub struct CoffeeMachine {
    // these values should not be reseted after shutdown
    global_coffees_made: u32,
    file_location: String,
}

impl CoffeeMachine {
    
    pub fn init() -> (Self, RecipeInterface, Resources) {
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
        if resources.check_resources(&recipe) {
            println!("grinding coffee...");

            resources.increment_ground_coffee_counter();

            self.global_coffees_made += 1;

            resources.amount_of_coffee_beans_g -= recipe.coffee_dosage_g;
            resources.amount_of_water_ml -= recipe.water_dosage_ml;

        } else {
            println!("Not enough resources to do a coffee");
        }
    }

    pub fn heat_water() {}

    pub fn add_hot_water() {}

    pub fn brew() {}
}


#[derive(Debug)]
pub struct RecipeInterface {
    water_dosage_ml: u32, // default values: short: 50ml, long: 100ml
    coffee_dosage_g: u32, // varies from 10 to 50 in grams
    double: bool,
}

impl RecipeInterface {

    // Associated functions

    pub fn init() -> Self {
        Self {
            water_dosage_ml: 0,
            double: false,
            coffee_dosage_g: 25,
        }
    }
    
    // Public methods

    pub fn set_double (&mut self, double: bool){
        self.double = double;
    }

    pub fn set_coffee_dosage(&mut self, level: String) {
        // matches the input and coerces it to a safe value
        let lower_limit = 1;
        let upper_limit = 15;

        self.coffee_dosage_g = match level.trim().parse() {
            Ok(num) if (lower_limit..=upper_limit).contains(&num) => num,
            Ok(_) => {
                println!("Input number is out of range. Accepted range is {lower_limit}..{upper_limit} (included). Input coerced to {}.", {upper_limit / 2 as u32});
                upper_limit / 2 as u32
            }
            Err(_) => {
                println!("Input not recognized. Accepted range is {lower_limit}..{upper_limit} (included). Input coerced to {}.", {upper_limit / 2 as u32});
                upper_limit / 2 as u32
            }
        };
    }
    
    pub fn update_recipe(&mut self) {
        // Updates the water and ground coffee amount
        self.set_water_amount_ml();
        self.doubles_coffee_and_water_dosage_g();
    }

    // Private methods

    fn set_water_amount_ml(&mut self){
        // adds 1.5 times the water as the coffee
        self.water_dosage_ml += self.coffee_dosage_g*1.5 as u32
    }

    fn doubles_coffee_and_water_dosage_g(&mut self) {
        // depending on the coffee dosage (strongness) and if a double is selected or not updates the coffee dosage      
        if self.double {
            // if a double cofee is selected, increase dosage by 50%
            self.coffee_dosage_g *= 1.5 as u32
        }
    }
}

#[derive(Debug)]
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
        self.amount_of_residues_g += recipe.coffee_dosage_g
    }

    pub fn check_resources(&mut self, recipe: &RecipeInterface) -> bool {
        // returns true if more than requested amount of beans, and
        // more than requested amount of water, and
        // less than 1000g of residues

        // TODO return which resource is missing by altering reference in place
        self.amount_of_coffee_beans_g > recipe.coffee_dosage_g
            && self.amount_of_water_ml > recipe.water_dosage_ml
            && self.amount_of_residues_g < 1000
    }

    fn increment_ground_coffee_counter(&mut self) {
        self.current_coffees_ground += 1;
    }
}
