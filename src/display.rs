pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const SCREEN_SIZE: usize = WIDTH * HEIGHT;

pub struct Display {
    buffer: [[bool; WIDTH]; HEIGHT],
}
impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; WIDTH]; HEIGHT],
        }
    }
    pub fn clear(&mut self) {
        self.buffer = [[false; WIDTH]; HEIGHT];
    }
    pub fn get_pixel(&self, y: usize, x: usize) -> bool {
        self.buffer[y][x]
    }
    pub fn flip_pixel(&mut self, y: usize, x: usize) {
        self.buffer[y][x] = !self.buffer[y][x];
    }
}
