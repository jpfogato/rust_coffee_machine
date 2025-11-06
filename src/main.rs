pub mod controller;
use crate::controller::CoffeeMachine;

fn main() {
    // initializing the machine, the recipe and the resources
    let (mut coffe_machine, mut recipe, mut resources) = CoffeeMachine::init();

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

    let test_vector = binbin_tester();
    println!("{:?}", test_vector);
}

fn binbin_tester() -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    binbin::write_vec_le(&mut buf, |w| {
        let header_start = w.position()? as u32;
        let header_len = w.deferred(0 as u32);
        w.write(&b"\x7fELF"[..])?;
        w.write(1 as u8)?; // 32-bit ELF
        w.write(1 as u8)?; // Little-endian ELF
        w.write(1 as u8)?; // ELF header version
        w.write(0 as u8)?; // ABI
        w.skip(8)?;
        w.write(1 as u16)?; // Relocatable
        w.write(0x28 as u16)?; // ARM instruction set
        w.write(1 as u32)?; // ELF version
        w.write(0 as u32)?; // no entry point
        w.write(0 as u32)?; // no program header table
        let section_header_pos = w.write_deferred(0 as u32)?;
        w.write(0 as u32)?; // flags
        w.write_placeholder(header_len)?;
        w.write(0 as u32)?; // size of program header entry (none)
        w.write(0 as u32)?; // number of program header entries (none)
        let section_header_size = w.write_deferred(0 as u32)?;
        let section_header_count = w.write_deferred(0 as u32)?;
        let header_end = w.position()? as u32;
        w.resolve(header_len, header_end - header_start);
        w.write(0 as u32)?; // no string table

        w.align(4)?;
        let pos = w.position()? as u32;
        w.resolve(section_header_pos, pos)?;

        // (...and then the rest of an ELF writer...)
        Ok(())
    })
    .expect("error using binbin");
    buf
}
