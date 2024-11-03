use std::collections::HashMap;
use std::sync::OnceLock;

use shorekeeper_data::LevelEntityConfigData;

#[derive(Clone)]
struct MapBounds {
    x_max: f32,
    x_min: f32,
    x_translate: f32,
    y_max: f32,
    y_min: f32,
    y_translate: f32,
}

#[derive(Default)]
struct Quadrant {
    entities: HashMap<i64, &'static LevelEntityConfigData>,
}

pub struct Map {
    bounds: MapBounds,
    width: u64,
    height: u64,
    quadrants: HashMap<u64, Quadrant>,
}

// TODO: Make it configurable?
const EDGE_SIZE: f32 = 1000000f32;
const EDGE_CHECK: f32 = EDGE_SIZE * 3.0f32;

pub(crate) static MAP_TABLE: OnceLock<HashMap<i32, Map>> = OnceLock::new();

impl MapBounds {
    fn find_max_min(slice: &[&LevelEntityConfigData]) -> (Self, bool) {
        let mut x_max = 0f32;
        let mut x_min = 0f32;

        let mut y_max = 0f32;
        let mut y_min = 0f32;

        // Find max and min coordinates
        for entity in slice.iter() {
            if entity.transform[0].x < x_min { x_min = entity.transform[0].x }
            if entity.transform[0].x > x_max { x_max = entity.transform[0].x }

            if entity.transform[0].y < y_min { y_min = entity.transform[0].y }
            if entity.transform[0].y > y_max { y_max = entity.transform[0].y }
        }

        if (f32::abs(x_max - x_min) < EDGE_CHECK) || (f32::abs(y_max - y_min) < EDGE_CHECK) {
            // TODO: Handle this special case, since all entities fit, no need for quadrant

            // Move everything to positive coordinates to prevent corner cases
            let (x_max, x_min, x_translate) = recenter_map(x_max, x_min);
            let (y_max, y_min, y_translate) = recenter_map(y_max, y_min);

            (MapBounds { x_max, x_min, x_translate, y_max, y_min, y_translate }, false)
        } else {
            // Round to edge
            x_max = round_max_coordinate(x_max, EDGE_SIZE);
            x_min = round_min_coordinate(x_min, EDGE_SIZE);
            y_max = round_max_coordinate(y_max, EDGE_SIZE);
            y_min = round_min_coordinate(y_min, EDGE_SIZE);

            // Adding bounds to prevent OOB when moving
            x_max += EDGE_SIZE;
            x_min -= EDGE_SIZE;
            y_max += EDGE_SIZE;
            y_min -= EDGE_SIZE;

            // Move everything to positive coordinates to prevent corner cases
            let (x_max, x_min, x_translate) = recenter_map(x_max, x_min);
            let (y_max, y_min, y_translate) = recenter_map(y_max, y_min);

            (MapBounds { x_max, x_min, x_translate, y_max, y_min, y_translate }, true)
        }
    }
}

impl Quadrant {
    fn insert_entity(&mut self, entity_id: i64, entity: &'static LevelEntityConfigData) {
        self.entities.insert(entity_id, entity);
    }

    fn get_entities(&self) -> Vec<&LevelEntityConfigData> {
        self.entities
            .iter()
            .map(|(_, v)| *v)
            .collect()
    }
}

impl Map {
    fn insert_entity(&mut self, entity: &'static LevelEntityConfigData) {
        let index = self.get_quadrant_id(entity.transform[0].x, entity.transform[0].y);
        self.quadrants.entry(index).or_default().insert_entity(entity.entity_id, entity)
    }

    fn get_neighbour_cells(&self, quadrant_id: u64) -> [u64; 9] {
        let x = quadrant_id % self.width;
        let y = (quadrant_id - x) / self.width;
        return [
            (self.width * (y - 1)) + (x - 1),
            (self.width * (y - 1)) + (x),
            (self.width * (y - 1)) + (x + 1),
            (self.width * (y)) + (x - 1),
            (self.width * (y)) + (x),
            (self.width * (y)) + (x + 1),
            (self.width * (y + 1)) + (x - 1),
            (self.width * (y + 1)) + (x),
            (self.width * (y + 1)) + (x + 1),
        ];
    }

    fn collect_quadrant_differences(&self, discriminant: [u64; 9], discriminator: [u64; 9]) -> Vec<&LevelEntityConfigData> {
        let mut output = Vec::new();
        for quadrant in discriminant {
            if !discriminator.contains(&quadrant) {
                if let Some(quadrant) = &self.quadrants.get(&quadrant) {
                    output.extend_from_slice(&quadrant.get_entities())
                }
            }
        }
        output
    }

    pub fn get_quadrant_id(&self, x: f32, y: f32) -> u64 {
        let width: u64 = unsafe {
            f32::to_int_unchecked(
                f32::trunc(
                    (self.bounds.x_max + self.bounds.x_translate - x) / EDGE_SIZE
                )
            )
        };
        let height: u64 = unsafe {
            f32::to_int_unchecked(
                f32::trunc(
                    (self.bounds.y_max + self.bounds.y_translate - y) / EDGE_SIZE
                )
            )
        };
        (self.width * height) + width
    }

    pub fn get_initial_entities(&self, quadrant_id: u64) -> Vec<&LevelEntityConfigData> {
        let quadrants = self.get_neighbour_cells(quadrant_id);
        let mut output = Vec::new();
        for quadrant in quadrants {
            if let Some(quadrant) = &self.quadrants.get(&quadrant) {
                output.extend_from_slice(&quadrant.get_entities())
            }
        }
        output
    }

    pub fn get_update_entities(&self, old_quadrant_id: u64, new_quadrant_id: u64) -> (Vec<&LevelEntityConfigData>, Vec<&LevelEntityConfigData>) {
        let old_quadrants = self.get_neighbour_cells(old_quadrant_id);
        let new_quadrants = self.get_neighbour_cells(new_quadrant_id);

        let entities_to_remove = self.collect_quadrant_differences(old_quadrants, new_quadrants);
        let entities_to_add = self.collect_quadrant_differences(new_quadrants, old_quadrants);

        (entities_to_remove, entities_to_add)
    }
}

pub fn maps_iter() -> std::collections::hash_map::Iter<'static, i32, Map> {
    MAP_TABLE.get().unwrap().iter()
}

pub fn initialize_quadrant_system() {
    let mut map_grouped_entities: HashMap<i32, Vec<&LevelEntityConfigData>> = HashMap::new();
    for (_, entity) in shorekeeper_data::level_entity_config_data::iter() {
        map_grouped_entities.entry(entity.map_id).or_default().push(entity);
    }

    let mut maps: HashMap<i32, Map> = HashMap::new();
    for (map_id, entities) in map_grouped_entities {
        let (bounds, _quadrant_enabled) = MapBounds::find_max_min(&entities[..]);
        let width = unsafe { f32::to_int_unchecked((bounds.x_max - bounds.x_min) / EDGE_SIZE) };
        let height = unsafe { f32::to_int_unchecked((bounds.y_max - bounds.y_min) / EDGE_SIZE) };
        let map = maps.entry(map_id).or_insert(
            Map {
                bounds: bounds.clone(),
                width,
                height,
                quadrants: HashMap::new(),
            }
        );

        for entity in entities {
            map.insert_entity(entity);
        }
    }

    let _ = MAP_TABLE.set(maps);
}

pub fn get_map(map_id: i32) -> &'static Map {
    // TODO: Error check for map id
    MAP_TABLE.get().unwrap().get(&map_id).unwrap()
}

fn recenter_map(max: f32, min: f32) -> (f32, f32, f32) {
    match min < 0.0 {
        true => (max + f32::abs(min), 0.0, min),
        false => (max, min, 0.0)
    }
}

fn round_max_coordinate(coordinate: f32, round: f32) -> f32 {
    let rounded = f32::round(coordinate);
    let remainder = rounded % round;
    if remainder != 0f32 {
        rounded + (if rounded > 0.0 { round } else { 0.0 } - remainder)
    } else {
        rounded
    }
}

fn round_min_coordinate(coordinate: f32, round: f32) -> f32 {
    let rounded = f32::round(coordinate);
    let remainder = rounded % round;
    if remainder != 0f32 {
        rounded + (remainder.signum() * (if rounded > 0.0 { 0.0 } else { round } - f32::abs(remainder)))
    } else {
        rounded
    }
}