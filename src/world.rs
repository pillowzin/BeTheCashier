use macroquad::prelude::*;
use std::collections::HashMap;

pub struct World {
    tileset: Texture2D,
    tilemap: HashMap<i32, (i32, i32)>,
    map: Vec<Vec<i32>>,
}

pub const TILE_SIZE: f32 = 16.0;
const TILESET_COLUMNS: i32 = 4;
const HORIZONTAL_WALL_TILE: i32 = tile_id_from_tileset_position(1, 3);
const VERTICAL_WALL_TILE: i32 = tile_id_from_tileset_position(3, 1);
const SIDE_WALL_TILE: i32 = 8;
const TOP_LEFT_CORNER_TILE: i32 = tile_id_from_tileset_position(0, 1);
const TOP_RIGHT_CORNER_TILE: i32 = tile_id_from_tileset_position(2, 1);

const fn tile_id_from_tileset_position(x: i32, y: i32) -> i32 {
    y * TILESET_COLUMNS + x
}

// const MAP: [[i32; 6]; 9] = [
//     [4, 13, 13, 13, 13, 6],
//     [8,  2,  2,  2,  2,  8],
//     [8,  0,  0,  0,  0,  8],
//     [8,  0,  1,  1,  0,  8],
//     [8,  0,  1,  1,  0,  8],
//     [8,  0,  1,  1,  0,  8],
//     [8,  0,  1,  1,  0,  8],
//     [8,  0,  0,  0,  0,  8],
//     [12,13,13,13,13,14],
// ];

fn generate_map(width: usize) -> Vec<Vec<i32>> {
    let base = [
        [4, 13, 13, 13, 13, 6],
        [8,  2,  2,  2,  2,  8],
        [8,  0,  0,  0,  0,  8],
        [8,  0,  1,  1,  0,  8],
        [8,  0,  1,  1,  0,  8],
        [8,  0,  1,  1,  0,  8],
        [8,  0,  1,  1,  0,  8],
        [8,  0,  0,  0,  0,  8],
        [12,13,13,13,13,14],
    ];

    let mut map = Vec::new();

    for row in base.iter() {
        let mut new_row = Vec::new();

        let left = row[0];
        let right = row[row.len() - 1];

        let inner = &row[1..row.len() - 1];

        let left_inner = inner[0];
        let right_inner = inner[inner.len() - 1];
        let middle = &inner[1..inner.len() - 1];

        new_row.push(left);
        new_row.push(left_inner);

        for i in 0..(width - 4) {
            new_row.push(middle[i % middle.len()]);
        }

        new_row.push(right_inner);
        new_row.push(right);

        map.push(new_row);
    }

    map
}

impl World {
    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub async fn new() -> Self {
        let tileset = load_texture("map/tileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let json_str = load_string("map/tileset-data.json").await.unwrap();
        let data: Vec<serde_json::Value> = serde_json::from_str(&json_str).unwrap();

        let mut tilemap = HashMap::new();

        for tile in data {
            let id = tile["id"].as_i64().unwrap() as i32;
            let x = tile["x"].as_i64().unwrap() as i32;
            let y = tile["y"].as_i64().unwrap() as i32;

            tilemap.insert(id, (x, y));
        }

        let map = generate_map(80);

        Self { tileset, tilemap, map }
    }

    pub fn is_solid_tile_at_world(&self, position: Vec2, size: Vec2) -> bool {
        let left = (position.x / TILE_SIZE).floor() as i32;
        let top = (position.y / TILE_SIZE).floor() as i32;
        let right = ((position.x + size.x - 1.0) / TILE_SIZE).floor() as i32;
        let bottom = ((position.y + size.y - 1.0) / TILE_SIZE).floor() as i32;

        for tile_y in top..=bottom {
            for tile_x in left..=right {
                if self.is_solid_tile(tile_x, tile_y) {
                    return true;
                }
            }
        }

        false
    }

    fn is_solid_tile(&self, tile_x: i32, tile_y: i32) -> bool {
        if tile_x < 0
            || tile_y < 0
            || tile_y as usize >= self.map.len()
            || tile_x as usize >= self.map[tile_y as usize].len()
        {
            return true;
        }

        matches!(
            self.map[tile_y as usize][tile_x as usize],
            HORIZONTAL_WALL_TILE
                | VERTICAL_WALL_TILE
                | SIDE_WALL_TILE
                | TOP_LEFT_CORNER_TILE
                | TOP_RIGHT_CORNER_TILE
        )
    }

    fn draw_tile_id(&self, id: i32, x: f32, y: f32) {
        if let Some(&(tx, ty)) = self.tilemap.get(&id) {
            self.draw_tile(tx, ty, x, y);
        }
    }

    fn draw_tile(&self, tile_x: i32, tile_y: i32, x: f32, y: f32) {
        let size = TILE_SIZE;

        draw_texture_ex(
            &self.tileset,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    tile_x as f32 * size,
                    tile_y as f32 * size,
                    size,
                    size,
                )),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }

    pub fn draw(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, id) in row.iter().enumerate() {
                let px = x as f32 * TILE_SIZE;
                let py = y as f32 * TILE_SIZE;

                self.draw_tile_id(*id, px, py);
            }
        }
    }
}
