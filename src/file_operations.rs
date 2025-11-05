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

#[cfg(test)]
pub mod file_handler {

    use std::cmp::{Eq, PartialEq};
    use std::collections::HashMap;
    use std::fs::{self, create_dir_all};
    use std::path::{self, Path, PathBuf};

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

    #[derive(Debug, PartialEq)]
    pub struct FileHandler {
        // create a variable with a MemoryMap type
        file_offsets: HashMap<FileOffsets, (i32, u8)>,
        file_path: PathBuf,
        buffer: [u8; 128],
    }

    impl FileHandler {
        // initialize the FileHandler data with the file offsets
        pub fn new() -> FileHandler {
            let mut new = FileHandler {
                buffer: [0; 128],
                file_offsets: Self::create_offset_map(),
                file_path: Path::new("/tmp/coffee_machine/runtime.bin").to_owned(),
            };
            new.initialize_buffer();
            new.retrieve_stored_data();

            new
        }

        // Returs a map of K: FileOffset enum, V: (index, size)
        fn create_offset_map() -> HashMap<FileOffsets, (i32, u8)> {
            let mut mapped_file_offsets = HashMap::new();
            mapped_file_offsets.insert(FileOffsets::Endianess, (3, 1));
            mapped_file_offsets.insert(FileOffsets::NumberOfCoffessBrewed, (33, 4));
            mapped_file_offsets.insert(FileOffsets::NeedsCoffeeBeans, (37, 1));
            mapped_file_offsets.insert(FileOffsets::NeedsWater, (38, 1));
            mapped_file_offsets.insert(FileOffsets::NeedsGroundsRemoval, (39, 1));
            mapped_file_offsets.insert(FileOffsets::NeedsDescaling, (40, 1));
            mapped_file_offsets.insert(FileOffsets::DefaultWaterDosage, (46, 4));
            mapped_file_offsets.insert(FileOffsets::DefaultCoffeeDosage, (50, 4));

            mapped_file_offsets
        }

        // Updates data from the live buffer with 'data' at the 'location'
        pub fn update_params_on_buffer(&mut self, data: u64, location: FileOffsets) {
            // TODO: Find a better way of generalizing the data input of this function.
            // Ideally the caller should not care about the data that it is sending to
            // it as long as it is a numeric or boolean value.
            match self.file_offsets.get(&location) {
                Some((start_offset, size)) => {
                    let data = data.to_le_bytes(); // TODO: read from buffer to check if file
                                                   // is LE or BE
                    let mut n: i32 = 0;
                    let mut offset: i32 = 0;
                    while offset < *size as i32 {
                        offset = start_offset + n;
                        self.buffer[offset as usize] = data[n as usize];
                        n += 1;
                    }
                    self.update_runtime_file();
                }
                None => {
                    todo!()
                }
            };
        }

        // Checks storage for last session file, if not found, returns a default value and write it to
        // disk. Updates the live buffer with the data retrieved or with those default values.
        fn retrieve_stored_data(&mut self) {
            match fs::read_to_string(&self.file_path) {
                Ok(data) => {
                    self.overwrite_buffer_with_string(data);
                }
                Err(_) => {
                    // an error here means that the file does not exists in the disk
                    // solve that by creating a new empty file
                    let folder_name = Path::parent(&self.file_path)
                        .expect("Unable to extract parent from file_path");
                    fs::create_dir_all(folder_name).expect("Unable to create folder");
                    fs::write(&self.file_path, [0; 128])
                        .expect("unable to write an empty byte array to the file");
                    self.initialize_buffer();
                }
            }
        }

        // Updates the whole buffer at once with the string read from the file
        fn overwrite_buffer_with_string(&mut self, data: String) {
            let data = data.as_bytes();
            self.set_buffer(data);
        }

        // returns an instance of the buffer type with a zeroed u32 array of 128 elements
        fn initialize_buffer(&mut self) {
            self.buffer = [0; 128];
        }

        // Non-destructive getter for buffer
        pub fn get_entire_buffer(&self) -> &[u8; 128] {
            &self.buffer
        }

        pub fn get_value_from_buffer(&self, what: FileOffsets) -> u64 {
            let value = 1;

            //self.buffer[self.file_offsets.get(what)..8];

            value
        }

        fn set_buffer(&mut self, data: &[u8]) {
            self.buffer = data.try_into().expect("buffer shall have 128 elements!");
        }

        // writes the current buffer in the file
        fn update_runtime_file(&self) {
            println!("file should be updated now...");
        }
    }

    impl Default for FileHandler {
        fn default() -> Self {
            Self::new()
        }
    }
}
