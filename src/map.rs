use core::{Map, MapConfig};

use bevy::{
    app::{Plugin, Startup, Update},
    color::palettes::css::WHITE,
    math::{vec2, Vec2},
    prelude::{Camera2dBundle, Commands, Gizmos, Res, ResMut, Resource},
};

//mod core;
pub mod core;

pub struct MapPlugin {
    pub draw: bool,
}
const CELL_SIZE: u8 = 100;

#[derive(Resource, Default)]
pub struct MapData {
    pub walls: Vec<(Vec2, Vec2)>
}

fn draw_map(
    mut commands: Commands,
    mut map_res: ResMut<MapData>
) {
    commands.spawn(Camera2dBundle::default());

    let map_size = MapPlugin::calculate_map_size(5, 3);

    let map_config = MapConfig {
        width: map_size.0,
        height: map_size.1,
        ..Default::default()
    };

    let mut map = Map::new(map_config);
    map.generate_map();

    map_res.walls = map.get_walls();
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    map_res: Res<MapData>
) {
    for wall in map_res.walls.clone() {
        gizmos.line_2d(
            (wall.0 - vec2(2.0, 2.0)) * CELL_SIZE as f32,
            (wall.1 - vec2(2.0, 2.0)) * CELL_SIZE as f32,
            WHITE
        );
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MapData>();
        if self.draw {
            app.add_systems(Startup, draw_map);
            app.add_systems(Update, draw_gizmos);
        }
    }
}

impl MapPlugin {
    fn calculate_map_size(player_count: usize, player_space: usize) -> (usize, usize) {
        let cells_on_one_player = player_space * player_space;

        let player_count = if player_count % 2 == 1 {
            player_count + 1
        } else {
            player_count
        };

        //let cells = cells_on_one_player * player_count;

        let divisor = (1..=(player_count as f32).sqrt().floor() as usize).filter_map(|i| {
            if player_count % i == 0 {
                Some(i)
            } else {
                None
            }
        }).last().unwrap();

        let width = player_count / divisor * player_space;
        let height = divisor * player_space;

        (width, height)
    }

    pub fn get_players_spawn_cells(&self) -> Vec<Vec2> {
        todo!()
    }

    pub fn get_ability_spawn_cell(&self) -> Vec2 {
        todo!()
    }
}