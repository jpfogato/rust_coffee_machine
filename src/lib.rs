/* This is the file handling module.
* Exposes an API that allows for a binary file to be written to a set location on disk, and then
* retrieved for setting the machine's standard values.
* It implements a custom binary format defined by the following table:
* offset (bytes)   |   data_type   |   comment
*   0   |   u8  | version major
*   1   |   u8  | version minor
*   2   |   u8  | version bugfix
*   3   |   u8  | endianess; 0 = little, 1 = big endian
*   4   |   u64 | serial number
*   12..32 | -- | space for future permanent tracking expansion
*   33  |   u32 | number of coffess made
*   37  |   u8  | needs coffee beans 0b11111111 or 0b00000000 for T|F
*   38  |   u8  | needs water 0b11111111 or 0b00000000 for T|F
*   39  |   u8  | needs grounds removal b11111111 or 0b00000000 for t|f
*   40  |   u8  | needs descaling 0b11111111 or 0b00000000 for T|F
*   41  |   u32 | active brew time in minutes
*   45  |   u32 | time in minutes since last coffee brewed
*   46  |   u32 | default water dosage in ml
*   50  |   u32 | default coffee dosage in grams
*   54..128 |   -- | space for future status tracking expansion
*/

pub mod file_operations {

    // Memory map with the items that must be written to the file so it is parsed appropriately
    #[derive(Debug)]
    struct MemoryMap {
        // description: (offset, data)
        endianess: (u32, u8),
        number_of_coffess_brewed: (u32, u32),
        needs_coffee_beans: (u32, u8),
        needs_water: (u32, u8),
        needs_grounds_removal: (u32, u8),
        needs_descaling: (u32, u8),
        default_water_dosage: (u32, u32),
        default_coffee_dosage: (u32, u32),
    }

    fn get_file_offsets() -> MemoryMap {
        MemoryMap {
            endianess: (3, 0),
            number_of_coffess_brewed: (33, 0),
            needs_coffee_beans: (37, 0),
            needs_water: (38, 0),
            needs_grounds_removal: (39, 0),
            needs_descaling: (40, 0),
            default_water_dosage: (46, 0),
            default_coffee_dosage: (50, 0),
        }
    }

    // Buffer management
    #[derive(Debug)]
    struct Buffer {
        buffer: [u8; 128],
    }

    impl Buffer {
        // returns an instance of the buffer type
        pub fn initialize_buffer() -> Buffer {
            return Self { buffer: [0; 128] };
        }

        // TODO: replaces the buffer with the element of MemoryMap at that position
        pub fn push_to_buffer(&mut self, data: MemoryMap) {}
    }

    // TODO: Read from file operations
    #[derive(Debug)]
    pub struct ReadFromFile {}
    impl ReadFromFile {}

    // TODO: Write to file operations
    #[derive(Debug)]
    pub struct WriteToFile {}
    impl WriteToFile {}
}
