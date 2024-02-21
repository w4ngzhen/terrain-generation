use std::collections::HashMap;
use image::ImageBuffer;
use crate::map_noise_generator::MapNoiseGenerator;

#[derive(Eq, PartialEq, Hash)]
enum Biome {
    // 0 - 100
    Ocean,
    // 100 - 120
    Beach,
    // e > 800
    Scorched,
    Bare,
    Tundra,
    Snow,
    // 600 - 800
    TemperateDesert,
    ShrubLand,
    Taiga,
    // 300 - 600
    Grassland,
    TemperateDeciduousForest,
    TemperateRainForest,
    // 600 - 800
    SubtropicalDesert,
    TropicalSeasonalForest,
    TropicalRainForest,
}

#[derive(Clone, Copy)]
struct MapColumn {
    elevation: u64,
    humidity: f64,
}

pub struct Map {
    width: u64,
    height: u64,
    // width * height
    columns: Vec<MapColumn>,
    biome_color_map: HashMap<Biome, [u8; 3]>,
}

impl Map {
    pub fn new(width: u64, height: u64) -> Self {
        let noise_generator = MapNoiseGenerator::new(0, 0);
        let size = (width * height) as usize;
        let mut columns = Vec::with_capacity(size);
        for x in 0..width {
            for y in 0..height {
                let normalized_x = x as f64 / width as f64;
                let normalized_y = y as f64 / height as f64;
                // generate elevation
                let elevation_val = noise_generator.elevation_noise(normalized_x, normalized_y);
                let elevation = (elevation_val * 1000.0).floor() as u64;
                // generate humidity
                let humidity = noise_generator.humidity_noise(normalized_x, normalized_y);
                // set
                columns.push(MapColumn {
                    elevation,
                    humidity,
                });
            }
        }
        // biome color map
        let biome_color_map = HashMap::from([
            // 0 - 100
            (Biome::Ocean, [68, 70, 121]),
            // 100 - 120
            (Biome::Beach, [160, 144, 121]),
            // e > 800
            (Biome::Scorched, [85, 85, 85]),
            (Biome::Bare, [136, 136, 136]),
            (Biome::Tundra, [187, 187, 171]),
            (Biome::Snow, [221, 221, 228]),
            // 600 - 800
            (Biome::TemperateDesert, [201, 209, 158]),
            (Biome::ShrubLand, [137, 153, 121]),
            (Biome::Taiga, [154, 169, 122]),
            // 300 - 600
            (Biome::Grassland, [137, 169, 90]),
            (Biome::TemperateDeciduousForest, [105, 147, 92]),
            (Biome::TemperateRainForest, [71, 135, 87]),
            // 600 - 800
            (Biome::SubtropicalDesert, [210, 185, 142]),
            (Biome::TropicalSeasonalForest, [87, 152, 73]),
            (Biome::TropicalRainForest, [54, 119, 86]),
        ]);
        Map {
            width,
            height,
            columns,
            biome_color_map,
        }
    }

    pub fn get_biome(&self, x: u64, y: u64) -> Biome {
        let idx = self.map_idx(x, y);
        let column = &self.columns[idx];
        let e = column.elevation;
        let humidity = column.humidity;
        if e < 100 { return Biome::Ocean; }
        if e < 120 { return Biome::Beach; }
        if e > 800 {
            if humidity < 0.1 { return Biome::Scorched; }
            if humidity < 0.2 { return Biome::Bare; }
            if humidity < 0.5 { return Biome::Tundra; }
            return Biome::Snow;
        }

        if e > 600 {
            if humidity < 0.33 { return Biome::TemperateDesert; }
            if humidity < 0.66 { return Biome::ShrubLand; }
            return Biome::Taiga;
        }

        if e > 300 {
            if humidity < 0.16 { return Biome::TemperateDesert; }
            if humidity < 0.50 { return Biome::Grassland; }
            if humidity < 0.83 { return Biome::TemperateDeciduousForest; }
            return Biome::TemperateRainForest;
        }

        if humidity < 0.16 { return Biome::SubtropicalDesert; }
        if humidity < 0.33 { return Biome::Grassland; }
        if humidity < 0.66 { return Biome::TropicalSeasonalForest; }
        return Biome::TropicalRainForest;
    }

    fn map_idx(&self, x: u64, y: u64) -> usize {
        (y * self.width + x) as usize
    }

    pub fn write_to_file(&self, filename: &str) {
        use std::{fs, path::Path};

        // Create the output directory for the images, if it doesn't already exist
        let target_dir = Path::new("example_images/");

        if !target_dir.exists() {
            fs::create_dir(target_dir).expect("failed to create example_images directory");
        }

        //concatenate the directory to the filename string
        let directory: String = "example_images/".to_owned();
        let file_path = directory + filename;
        // collect the values from f64 into u8 in a separate vec

        let mut img_buf = ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let biome_opt = self.biome_color_map.get(&self.get_biome(x as u64, y as u64));
            if let Some(biome_color) = biome_opt {
                *pixel = image::Rgb(biome_color.clone());
            }
        }

        // Save the image as “fractal.png”, the format is deduced from the path
        img_buf.save(file_path).unwrap();
    }
}
