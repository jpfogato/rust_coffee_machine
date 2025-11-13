/* This is the file handling module.
* Exposes an API that allows for a binary file to be written to a set location on disk, and then
* retrieved for setting the machine's standard values.
* Check function "initialize_vector" to verify the following implementation
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
    use std::io::{Read, Write};
    use std::path::{self, Path, PathBuf};
    use std::string;

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
        runtime_params: Vec<u8>,
    }

    impl FileHandler {
        // initialize the FileHandler data with the file offsets
        pub fn new() -> FileHandler {
            let mut new = FileHandler {
                runtime_params: Self::initialize_vector(128),
                file_offsets: Self::create_offset_map(),
                file_path: Path::new("/tmp/coffee_machine/runtime.bin").to_owned(),
            };
            new.read_whole_file();

            new
        }

        // initializes the vector with default data
        pub fn initialize_vector(size: u64) -> Vec<u8> {
            let mut buf = Vec::<u8>::new();
            binbin::write_vec_le(&mut buf, |w| {
                let header_start = w.position()? as u32;
                let header_len = w.deferred(0 as u32);
                //w.write(&b"\x7fELF"[..])?;
                w.write(1 as u8)?; // version major
                w.write(0 as u8)?; // version minor
                w.write(0 as u8)?; // version bugfix
                w.write(0 as u8)?; // endianess -> little
                w.skip(20)?; // adds 20 bytes of padding here
                w.write(0 as u32)?; // number of coffees made
                w.write(0 as u8)?; // needs coffee flag
                w.write(0 as u8)?; // needs water flag
                w.write(0 as u8)?; // needs grounds removal flag
                w.write(0 as u8)?; // needs descaling flag
                w.write(0 as u32)?; // active brew time in minutes
                w.write(0 as u32)?; // time in minutes since last coffee brewed
                w.write(0 as u32)?; // default water dosage in mls
                w.write(0 as u32)?; // default coffee dosage in grams
                w.write(0 as u32)?; // size of program header entry (none)
                w.write(0 as u32)?; // number of program header entries (none)
                let file_end = w.position();
                let padding = size - file_end.unwrap_or_default();
                w.skip(padding as usize); // writes 74 bytes of padding to ensure buffer has 128 bytes
                Ok(())
            })
            .expect("error allocating vector");
            buf
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
        pub fn update_runtime_params(&mut self, data: String, location: FileOffsets) {
            match self.file_offsets.get(&location) {
                Some((start_offset, size)) => {
                    let data = data.as_bytes();
                    let mut n: i32 = 0;
                    let mut offset: i32 = 0;
                    while offset < *size as i32 {
                        offset = start_offset + n;
                        self.runtime_params[offset as usize] = data[n as usize];
                        n += 1;
                    }
                    self.write_whole_file();
                }
                None => {
                    todo!()
                }
            };
        }

        // writes the current buffer in the file
        fn write_whole_file(&mut self) {}

        // Updates the whole buffer at once with the data read from the file
        fn read_whole_file(&mut self) {
            // casts the folder dir from string to Path Type
            let parent_folder = Path::parent(&self.file_path);

            // attempts to open the file
            match fs::File::open(&self.file_path) {
                // if succeeded, try to read it
                Ok(mut file_refnum) => {
                    let mut buffer: Vec<u8> = vec![0; 128];
                    match file_refnum.read_to_end(&mut buffer) {
                        // there's something to read in the file,
                        // the buffer variable has been mutated in place, so we can ignore the Ok return
                        Ok(_) => {
                            self.runtime_params = buffer;
                        }
                        // else, panics and return the error
                        Err(e) => {
                            eprintln!("Error reading file: {}", e);
                        }
                    }
                }
                Err(_) => {
                    // an error here means that the file does not exists.
                    // first, try to create dir
                    fs::create_dir_all(parent_folder.unwrap()).expect("Unable to create folder");
                    // then try to create the file itself
                    let mut file =
                        fs::File::create_new(&self.file_path).expect("unable to create file");
                    let vector = FileHandler::initialize_vector(128);
                    file.write_all(&vector)
                        .expect("unable to write an initialized byte array to the file");
                }
            }
        }

        // Non-destructive getter for a slice of the run-time buffer
        fn get_buffer_slice(&self, offset: usize, length: usize) -> &[u8] {
            let length: usize = length + &offset;
            &self.runtime_params[offset..=length]
        }

        // retrieves a slice offseted by the FileOffsets from the vector with the live runtime data
        pub fn get_param(&self, what: FileOffsets) -> &[u8] {
            let item = &self
                .file_offsets
                .get(&what)
                .expect("Element not implemented");
            self.get_buffer_slice(item.0 as usize, item.1 as usize)
        }

        fn set_buffer(&mut self, data: &[u8]) {
            self.runtime_params = data.try_into().expect("buffer shall have 128 elements!");
        }
    }

    impl Default for FileHandler {
        fn default() -> Self {
            Self::new()
        }
    }
}
