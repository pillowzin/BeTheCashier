use crate::config::*;
use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    velocity: Vec2,
    texture: Texture2D,
    frame: i32,
    timer: f32,
    is_running: bool,
    facing_left: bool,
}

impl Player {
    pub async fn new() -> Self {
        let texture = load_texture("sprites/host.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            position: vec2(100.0, 100.0),
            velocity: Vec2::ZERO,
            texture,
            frame: 0,
            timer: 0.0,
            is_running: false,
            facing_left: false,
        }
    }

    fn get_current_frame(&self) -> Rect {
        let frame_width = 16.0;
        let frame_height = 16.0;

        let x = self.frame as f32 * frame_width;

        let y = if self.is_running { 16.0 } else { 0.0 };

        Rect::new(x, y, frame_width, frame_height)
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity = Vec2::ZERO;

        let was_running = self.is_running;
        self.is_running = false;

        if is_key_down(KeyCode::A) {
            self.velocity.x -= 1.0;
            self.facing_left = true;
        }

        if is_key_down(KeyCode::D) {
            self.velocity.x += 1.0;
            self.facing_left = false;
        }

        if is_key_down(KeyCode::W) {
            self.velocity.y -= 1.0;
        }

        if is_key_down(KeyCode::S) {
            self.velocity.y += 1.0;
        }

        if self.velocity.length() > 0.0 {
            self.is_running = true;
        }

        // reset de animação ao trocar estado (resolve flicker)
        if self.is_running != was_running {
            self.frame = 0;
            self.timer = 0.0;
        }

        self.position += self.velocity.normalize_or_zero() * PLAYER_SPEED * dt;

        // animação
        const ANIM_FPS: f32 = 10.0;
        let frame_time = 1.0 / ANIM_FPS;

        self.timer += dt;
        if self.timer >= frame_time {
            self.timer -= frame_time;
            self.frame += 1;

            let max_frames = 8;
            self.frame %= max_frames;
        }
    }

    pub fn clamp_to_scene(&mut self, scene_size: Vec2) {
        let min_x = BORDER_LEFT;
        let min_y = BORDER_TOP;
        let max_x = (scene_size.x - BORDER_RIGHT - PLAYER_SIZE).max(min_x);
        let max_y = (scene_size.y - BORDER_BOTTOM - PLAYER_SIZE).max(min_y);

        self.position.x = self.position.x.clamp(min_x, max_x);
        self.position.y = self.position.y.clamp(min_y, max_y);
    }

    pub fn is_moving(&self) -> bool {
        self.velocity.length_squared() > 0.0
    }

    pub fn draw(&self) {
        let visual_offset = (PLAYER_SIZE - PLAYER_VISUAL_SIZE) * 0.5;
        draw_texture_ex(
            &self.texture,
            (self.position.x + visual_offset).round(),
            (self.position.y + visual_offset).round(),
            WHITE,
            DrawTextureParams {
                source: Some(self.get_current_frame()),
                dest_size: Some(vec2(PLAYER_VISUAL_SIZE, PLAYER_VISUAL_SIZE)),
                flip_x: self.facing_left,
                ..Default::default()
            },
        );
    }
}
