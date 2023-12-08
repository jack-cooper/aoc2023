use std::{ops::ControlFlow, str::FromStr};

use aoc2023::solve_day;

fn main() {
    solve_day(5, part1, part2);
}

#[derive(Default)]
struct Almanac {
    fertilizer_to_water: Vec<CategoryMap>,
    humidity_to_location: Vec<CategoryMap>,
    light_to_temperature: Vec<CategoryMap>,
    seeds: Vec<i64>,
    seed_to_soil: Vec<CategoryMap>,
    soil_to_fertilizer: Vec<CategoryMap>,
    temperature_to_humidity: Vec<CategoryMap>,
    water_to_light: Vec<CategoryMap>,
}

struct CategoryMap {
    offset: i64,
    len: i64,
    source_start: i64,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Self::default();

        let mut lines = s.lines();

        let seeds = lines.next().expect("The input should not be empty.");

        let (_, seeds) = seeds
            .split_once(':')
            .expect("The seeds descriptor should contain a `:`");

        let seeds: Vec<i64> = seeds.trim_start().split(' ').flat_map(str::parse).collect();

        almanac.seeds = seeds;

        let mut category_map_list = &mut almanac.seed_to_soil;

        for line in lines.filter(|line| !line.is_empty()) {
            if line.contains("seed-to-soil map") {
                category_map_list = &mut almanac.seed_to_soil;
                continue;
            } else if line.contains("soil-to-fertilizer map") {
                category_map_list = &mut almanac.soil_to_fertilizer;
                continue;
            } else if line.contains("fertilizer-to-water map") {
                category_map_list = &mut almanac.fertilizer_to_water;
                continue;
            } else if line.contains("water-to-light map") {
                category_map_list = &mut almanac.water_to_light;
                continue;
            } else if line.contains("light-to-temperature map") {
                category_map_list = &mut almanac.light_to_temperature;
                continue;
            } else if line.contains("temperature-to-humidity map") {
                category_map_list = &mut almanac.temperature_to_humidity;
                continue;
            } else if line.contains("humidity-to-location map") {
                category_map_list = &mut almanac.humidity_to_location;
                continue;
            }

            let mut category_values = line.split(' ').flat_map(str::parse::<i64>);
            let destination_range_start = category_values
                .next()
                .expect("A category map must provide a destination range start.");
            let source_range_start = category_values
                .next()
                .expect("A category map must provide a source range start.");
            let range_length = category_values
                .next()
                .expect("A category map must provide a range length.");

            category_map_list.push(CategoryMap {
                offset: destination_range_start - source_range_start,
                len: range_length,
                source_start: source_range_start,
            })
        }

        Ok(almanac)
    }
}

impl Almanac {
    fn map_category(&self, mapping: CategoryMapping, source_values: &mut [i64]) {
        let category_map_list = match mapping {
            CategoryMapping::FertilizerToWater => &self.fertilizer_to_water,
            CategoryMapping::HumidityToLocation => &self.humidity_to_location,
            CategoryMapping::LightToTemperature => &self.light_to_temperature,
            CategoryMapping::SeedToSoil => &self.seed_to_soil,
            CategoryMapping::SoilToFertilizer => &self.soil_to_fertilizer,
            CategoryMapping::TemperatureToHumidity => &self.temperature_to_humidity,
            CategoryMapping::WaterToLight => &self.water_to_light,
        };

        source_values.iter_mut().for_each(|source_value| {
            category_map_list.iter().try_for_each(|category_map| {
                if category_map.contains(*source_value) {
                    *source_value += category_map.offset;
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
        });
    }

    fn seeds_to_locations(&self) -> Vec<i64> {
        let mut source_values = self.seeds.clone();

        self.map_category(CategoryMapping::SeedToSoil, &mut source_values);
        self.map_category(CategoryMapping::SoilToFertilizer, &mut source_values);
        self.map_category(CategoryMapping::FertilizerToWater, &mut source_values);
        self.map_category(CategoryMapping::WaterToLight, &mut source_values);
        self.map_category(CategoryMapping::LightToTemperature, &mut source_values);
        self.map_category(CategoryMapping::TemperatureToHumidity, &mut source_values);
        self.map_category(CategoryMapping::HumidityToLocation, &mut source_values);

        source_values
    }
}

impl CategoryMap {
    fn contains(&self, source_value: i64) -> bool {
        let range = self.source_start..(self.source_start + self.len);

        range.contains(&source_value)
    }
}

enum CategoryMapping {
    FertilizerToWater,
    HumidityToLocation,
    LightToTemperature,
    SeedToSoil,
    SoilToFertilizer,
    TemperatureToHumidity,
    WaterToLight,
}

fn part1(input: &str) -> u32 {
    let almanac: Almanac = input.parse().expect("Parsing an almanac can't fail.");

    let locations_for_seeds = almanac.seeds_to_locations();

    let min_location = locations_for_seeds
        .into_iter()
        .min()
        .expect("At least one location per seed will always be returned.");

    min_location as u32
}

fn part2(input: &str) -> u32 {
    let mut almanac: Almanac = input.parse().expect("Parsing an almanac can't fail.");

    0
}
