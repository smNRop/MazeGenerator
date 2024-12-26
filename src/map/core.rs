use bevy::math::{vec2, Vec2};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(PartialEq, Clone)]
#[derive(Debug)]
struct Connection(usize, usize);

impl Connection {
    pub fn new(from: usize, to: usize) -> Self {
        Connection(from.min(to), from.max(to))
    }
}

pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub seed: u64,
}

impl Default for MapConfig {
    fn default() -> Self {
        MapConfig {
            width: 3,
            height: 3,
            seed: 0
        }
    }
}

impl MapConfig {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

pub struct Map {
    map_config: MapConfig,
    path: Vec<Connection>,
    random: ChaCha8Rng
}

impl Map {
    pub fn new(map_config: MapConfig) -> Map {
        let random = ChaCha8Rng::seed_from_u64(map_config.seed);

        let map = Map {
            map_config,
            random,
            path: vec![]
        };

        map
    }

    pub fn get_config(&self) -> &MapConfig {
        &self.map_config
    }

    pub fn generate_map(&mut self) {
        let start_node = self.random.gen_range(0..self.map_config.height * self.map_config.width - 1);

        let mut visited_nodes = vec![];
        self.recursive_backtracking(start_node, &mut visited_nodes);
    }

    fn get_neighbors(&self, node: usize) -> Vec<usize> {
        let coords = self.get_node_coords(node);
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        directions.iter().filter_map(|&direction| {
            self.get_node_from_coords((coords.0 + direction.0, coords.1 + direction.1))
        }).collect::<Vec<_>>()
    }

    fn get_node_coords(&self, node: usize) -> (i32, i32) {
        (node as i32 % self.map_config.width as i32, node as i32 / self.map_config.width as i32)
    }

    fn get_node_from_coords(&self, coords: (i32, i32)) -> Option<usize> {
        if coords.0 < self.map_config.width as i32 && coords.0 >= 0 && coords.1 < self.map_config.height as i32 && coords.1 >= 0 {
            Some((coords.1 * self.map_config.width as i32 + coords.0) as usize)
        } else {
            None
        }
    }

    fn recursive_backtracking(&mut self, current_node: usize, visited_nodes: &mut Vec<usize>) {
        let mut neighbors = self.get_neighbors(current_node);
        neighbors.shuffle(&mut self.random);

        visited_nodes.push(current_node);

        for neighbor in neighbors {
            if visited_nodes.contains(&neighbor) {
                continue;
            }

            self.path.push(Connection::new(current_node, neighbor));
            self.recursive_backtracking(neighbor, visited_nodes);
        }
    }

    pub fn get_walls(&self) -> Vec<(Vec2, Vec2)> {
        let mut walls: Vec<(Vec2, Vec2)> = vec![];

        for i in 0..self.map_config.height * self.map_config.width {
            let neighbors = self.get_neighbors(i);

            for neighbor in neighbors {
                if self.path.contains(&Connection::new(i as usize, neighbor)) {
                    continue;
                }

                let (node_coords, neighbor_coords) = (
                    self.get_node_coords(i),
                    self.get_node_coords(neighbor)
                );

                let (v1, v2) = (
                    vec2(node_coords.0 as f32, node_coords.1 as f32),
                    vec2(neighbor_coords.0 as f32, neighbor_coords.1 as f32)
                );

                let perpendicular = (v2 - v1).perp().normalize() / 2.;
                let middle_point = (v2 + v1) / 2.;

                walls.push((
                    middle_point + perpendicular,
                    middle_point - perpendicular
                ));
            }
        }

        let width = self.map_config.width as f32 - 1.;
        let height = self.map_config.height as f32 - 1.;

        let left_bottom = vec2(0., 0.) + vec2(-0.5, -0.5);
        let right_bottom = vec2(width, 0.) + vec2(0.5, -0.5);
        let right_top = vec2(width, height) + vec2(0.5, 0.5);
        let left_top = vec2(0., height) + vec2(-0.5, 0.5);

        walls.push((left_bottom, right_bottom));
        walls.push((right_bottom, right_top));
        walls.push((right_top, left_top));
        walls.push((left_top, left_bottom));
        
        walls
    }
}