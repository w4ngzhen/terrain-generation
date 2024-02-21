use crate::map::Map;

mod map;
mod map_noise_generator;

fn main() {
    let map = Map::new(1024, 1024);
    map.write_to_file("test.png");
}
