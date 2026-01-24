pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    buffer: [bool; WIDTH * HEIGHT],
}
impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [false; WIDTH * HEIGHT],
        }
    }
    pub fn clear(&mut self) {
        self.buffer = [false; WIDTH * HEIGHT];
    }
    pub fn get_pixel(&self, y: usize, x: usize) -> bool {
        let pos = (y * WIDTH) + x;
        self.buffer[pos]
    }
    pub fn flip_pixel(&mut self, y: usize, x: usize) {
        let pos = (y * WIDTH) + x;
        self.buffer[pos] = !self.buffer[pos];
    }
}
