use std::fs::File;
use std::io::Read;

pub struct ROM {
    pub data: [u8; 3584],
}
impl ROM {
    pub fn new(rom_path: &str) -> ROM {
        let mut file = File::open(rom_path).expect("Unable to open ROM file");
        let mut buffer: [u8; 3584] = [0; 3584];
        file.read(&mut buffer).expect("Unable to read ROM file");

        ROM { data: buffer }
    }
}
