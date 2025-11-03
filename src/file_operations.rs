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

use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum FileOffsets {
    // TODO: define elements as they are required
    Endianess,
    NumberOfCoffessBrewed,
    NeedsCoffeeBeans,
    NeedsWater,
    NeedsGroundsRemoval,
    NeedsDescaling,
    DefaultWaterDosage,
    DefaultCoffeeDosage,
}

pub fn initialize_file_offsets() -> HashMap<FileOffsets, (i32, u8)> {
    // Returs a map of K: FileOffset enum, V: (index, size)
    let mut mapped_file_offsets = HashMap::new();
    mapped_file_offsets.insert(FileOffsets::Endianess, (3, 1));
    mapped_file_offsets.insert(FileOffsets::NumberOfCoffessBrewed, (33, 4));
    mapped_file_offsets.insert(FileOffsets::NeedsCoffeeBeans, (37, 1));
    mapped_file_offsets.insert(FileOffsets::NeedsWater, (38, 1));
    mapped_file_offsets.insert(FileOffsets::NeedsGroundsRemoval, (39, 1));
    mapped_file_offsets.insert(FileOffsets::NeedsDescaling, (40, 1));
    mapped_file_offsets.insert(FileOffsets::DefaultWaterDosage, (46, 4));
    mapped_file_offsets.insert(FileOffsets::DefaultCoffeeDosage, (50, 4));

    return mapped_file_offsets;
}

#[derive(Debug)]
pub struct FileHandler {
    // create a variable with a MemoryMap type
    file_offsets: HashMap<FileOffsets, (i32, u8)>,
    buffer: Buffer,
}
impl FileHandler {
    // initialize the FileHandler data with the file offsets
    pub fn initialize_file_handler() -> FileHandler {
        FileHandler {
            file_offsets: initialize_file_offsets(),
            buffer: Buffer::initialize_buffer(),
        }
    }

    // Updates data from the live buffer with 'data' at the 'location'
    fn update_live_buffer(&mut self, at_location: FileOffsets, with_data: u64) {
        // TODO: Find a better way of generalizing the data input of this function.
        // Ideally the caller should not care about the data that it is sending to
        // it as long as it is a numeric or boolean value.
        match self.file_offsets.get(&at_location) {
            Some((start_offset, size)) => {
                let data = with_data.to_le_bytes(); // TODO: read from buffer to check if file
                                                    // is LE or BE
                let mut n: i32 = 0;
                let mut offset: i32 = 0;
                while offset < *size as i32 {
                    offset = start_offset + n;
                    self.buffer.buffer[offset as usize] = data[n as usize];
                    n += 1;
                }
            }
            None => {
                todo!()
            }
        };
    }

    // Checks storage for last session file, if not found, returns a default value and write it to
    // disk. Updates the live buffer with the data retrieved or with those default values.
    fn retrieve_stored_data(&mut self) {}
}

// Buffer management
#[derive(Debug)]
struct Buffer {
    pub buffer: [u8; 128],
}

impl Buffer {
    // returns an instance of the buffer type with a zeroed u32 array of 128 elements
    pub fn initialize_buffer() -> Buffer {
        Self { buffer: [0; 128] }
    }

    fn get_buffer(&self) -> [u8; 128] {
        // getter for buffer
        self.buffer
    }
}
