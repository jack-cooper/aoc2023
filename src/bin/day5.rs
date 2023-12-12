use std::{
    ops::{ControlFlow, Range},
    str::FromStr,
};

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
    seed_ranges: Vec<Range<i64>>,
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

        almanac.seed_ranges = seeds
            .chunks_exact(2)
            .map(|seed_range| {
                let &[range_start, range_len] = seed_range else {
                    unreachable!();
                };

                range_start..(range_start + range_len)
            })
            .collect();

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
    fn location_to_seed(&self, location_value: i64) -> Option<i64> {
        let humidity_value =
            self.map_category_reverse(CategoryMappingReverse::LocationToHumidity, location_value);

        let temperature_value = self.map_category_reverse(
            CategoryMappingReverse::HumidityToTemperature,
            humidity_value,
        );

        let light_value = self.map_category_reverse(
            CategoryMappingReverse::TemperatureToLight,
            temperature_value,
        );

        let water_value =
            self.map_category_reverse(CategoryMappingReverse::LightToWater, light_value);

        let fertilizer_value =
            self.map_category_reverse(CategoryMappingReverse::WaterToFertilizer, water_value);

        let soil_value =
            self.map_category_reverse(CategoryMappingReverse::FertilizerToSoil, fertilizer_value);

        let seed_value = self.map_category_reverse(CategoryMappingReverse::SoilToSeed, soil_value);

        self.seed_ranges
            .iter()
            .any(|range| range.contains(&seed_value))
            .then_some(seed_value)
    }

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
                if category_map.contains_source(*source_value) {
                    *source_value += category_map.offset;
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
        });
    }

    fn map_category_reverse(&self, mapping: CategoryMappingReverse, source_value: i64) -> i64 {
        let category_map_list = match mapping {
            CategoryMappingReverse::FertilizerToSoil => &self.soil_to_fertilizer,
            CategoryMappingReverse::HumidityToTemperature => &self.temperature_to_humidity,
            CategoryMappingReverse::LightToWater => &self.water_to_light,
            CategoryMappingReverse::LocationToHumidity => &self.humidity_to_location,
            CategoryMappingReverse::SoilToSeed => &self.seed_to_soil,
            CategoryMappingReverse::TemperatureToLight => &self.light_to_temperature,
            CategoryMappingReverse::WaterToFertilizer => &self.fertilizer_to_water,
        };

        let mapped_value = category_map_list.iter().find_map(|category_map| {
            category_map
                .contains_dest(source_value)
                .then_some(source_value - category_map.offset)
        });

        mapped_value.unwrap_or(source_value)
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
    fn contains_dest(&self, source_value: i64) -> bool {
        let range = (self.source_start + self.offset)..(self.source_start + self.offset + self.len);

        range.contains(&source_value)
    }

    fn contains_source(&self, source_value: i64) -> bool {
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

enum CategoryMappingReverse {
    FertilizerToSoil,
    HumidityToTemperature,
    LightToWater,
    LocationToHumidity,
    SoilToSeed,
    TemperatureToLight,
    WaterToFertilizer,
}

fn part1(input: &str) -> u64 {
    let almanac: Almanac = input.parse().expect("Parsing an almanac can't fail.");

    let locations_for_seeds = almanac.seeds_to_locations();

    let min_location = locations_for_seeds
        .into_iter()
        .min()
        .expect("At least one location per seed will always be returned.");

    min_location as u64
}

fn part2(input: &str) -> u64 {
    let almanac: Almanac = input.parse().expect("Parsing an almanac can't fail.");

    (0..)
        .find_map(|location_value| {
            almanac
                .location_to_seed(location_value)
                .and(Some(location_value as u64))
        })
        .expect("`find_map` is being run on a range which is unbounded at the top.")
}
