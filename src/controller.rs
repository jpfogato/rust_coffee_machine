/*
Business-logic of the coffee-machine
*/

#[derive(Debug)]
pub enum Coarseness {
    Fine,
    Medium,
    Coarse,
}
#[derive(Debug)]
pub struct Recipe {
    pub water_amount_ml: u32,
    pub ground_coffee_amount_g: u32,
    pub brew_time_s: u32,
    pub coarseness: Coarseness,
    pub double: bool
}
pub struct Resources{
    pub amount_of_water_ml: u32,
    pub amount_of_coffee_beans_g: u32,
    pub amount_of_residues_g: u32,
}

pub fn sim_user_fill_water (amount_added_ml: u32, resources: &mut Resources) -> &mut Resources {
    resources.amount_of_water_ml = resources.amount_of_water_ml + amount_added_ml;
    println!("Water level is now: {} ml", &resources.amount_of_water_ml);
    resources
}

pub fn sim_user_add_beans () {}

pub fn grind () {}

pub fn dose_ground_coffee () {}

pub fn heat_water () {}

pub fn add_hot_water (recipe: Recipe) {
}

pub fn brew () {}

pub fn check_resources (resources: Resources) {}

pub fn warn_user (resources: Resources) {}