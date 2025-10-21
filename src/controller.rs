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
    needs_water: bool,
    needs_beans: bool,
    needs_grounds_removal: bool,
    coffee_ground: bool,
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
                needs_beans: false,
                needs_water: false,
                needs_grounds_removal: false,
                coffee_ground: false,
            },
            RecipeInterface::init(),
            Resources::init(),
        );
    }

    pub fn grind(&mut self, resources: &mut Resources, recipe: &mut RecipeInterface) {
        // first, check if the coffe can be ground
        self.coffee_ground = resources.check_resources(&recipe);

        // then grind the coffee, or warn if failed
        if self.coffee_ground {
            println!("grinding coffee...");
            self.global_coffees_made += 1;
            resources.amount_of_coffee_beans_g -= recipe.coffee_dosage_g;
        } else {
            println!("Not enough resources to do a coffee");
        }
    }

    pub fn brew(&mut self, resources: &mut Resources, recipe: &RecipeInterface) {
        if self.coffee_ground {
            println!("brewing coffee");
            resources.amount_of_water_ml -= recipe.water_dosage_ml;
            println!("coffee has been brewed!");
            self.coffee_ground = false; // return this to false so we cannot brew 2 coffees in a row
        } else {
            println!("grind the coffee beans first before brewing");
        }
    }

    pub fn needs_water(&self) -> bool {
        // get needs_water flag
        self.needs_water
    }

    pub fn file_location(&self) -> &str {
        &self.file_location
    }

    pub fn needs_beans(&self) -> bool {
        self.needs_beans
    }

    pub fn needs_grounds_removal(&self) -> bool {
        self.needs_grounds_removal
    }
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
    pub fn set_double(&mut self, double: bool) {
        self.double = double;
    }

    pub fn set_coffee_dosage(&mut self, level: String) {
        // matches the input and coerces it to a safe value
        let lower_limit = 10;
        let upper_limit = 20;

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

    fn set_water_amount_ml(&mut self) {
        // adds 2 times the water as the coffee
        self.water_dosage_ml += self.coffee_dosage_g * 2 as u32
    }

    fn doubles_coffee_and_water_dosage_g(&mut self) {
        // depending on the coffee dosage (strongness) and if a double is selected or not updates the coffee dosage
        if self.double {
            // if a double cofee is selected, increase dosage by 100%
            self.coffee_dosage_g *= 2 as u32;
            self.set_water_amount_ml();
        }
    }
}

#[derive(Debug)]
pub struct Resources {
    amount_of_water_ml: u32,
    amount_of_coffee_beans_g: u32,
    amount_of_residues_g: u32,
}

impl Resources {
    // Public methods

    pub fn init() -> Self {
        Self {
            amount_of_coffee_beans_g: 0,
            amount_of_residues_g: 0,
            amount_of_water_ml: 0,
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

    pub fn get_resource_amount(&self) -> (u32, u32) {
        (self.amount_of_coffee_beans_g, self.amount_of_water_ml)
    }
}
