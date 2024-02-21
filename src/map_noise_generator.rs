use noise::core::perlin::perlin_2d;
use noise::permutationtable::PermutationTable;

pub struct MapNoiseGenerator {
    elevation_hasher: PermutationTable,
    humidity_hasher: PermutationTable,
}

impl MapNoiseGenerator {
    pub fn new(elevation_seed: u32, humidity_seed: u32) -> Self {
        MapNoiseGenerator {
            elevation_hasher: PermutationTable::new(elevation_seed),
            humidity_hasher: PermutationTable::new(humidity_seed),
        }
    }

    pub fn elevation_noise(&self, x: f64, y: f64) -> f64 {
        let mut val = normalized_perlin_2d(x, y, &self.elevation_hasher);
        // + 0.5 * normalized_perlin_2d(x * 2.0, y * 2.0, &self.elevation_hasher);
        // + 0.25 * normalized_perlin_2d(x * 4.0, y * 4.0, &self.elevation_hasher);
        // val = val / (1.0 + 0.5);
        // val = val.powf(2.5);
        // if val > 0.8 {
        //     println!("e > 0.8: {}", val);
        // }
        val
    }

    pub fn humidity_noise(&self, x: f64, y: f64) -> f64 {
        let mut val = normalized_perlin_2d(x, y, &self.humidity_hasher)
            + 0.5 * normalized_perlin_2d(x * 2.0, y * 2.0, &self.humidity_hasher)
            + 0.25 * normalized_perlin_2d(x * 4.0, y * 4.0, &self.humidity_hasher);
        val = val / (1.0 + 0.5 + 0.25);
        val
    }
}

fn normalized_perlin_2d(x: f64, y: f64, hasher: &PermutationTable) -> f64 {
    // [-1.0, 1.0] -> [0, 1.0]
    let noise_val = perlin_2d([x, y], hasher) / 2.0 + 0.5;
    noise_val
}