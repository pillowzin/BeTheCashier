use macroquad::prelude::*;

mod config;
mod host;
mod world;

use world::World;
use config::*;
use host::Player;

const FIXED_DT: f32 = 1.0 / 60.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Be The Cashier".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new().await;

    // 🔥 carrega a cena
    let world = World::new().await;
    let scene_size = vec2(
        world.width() as f32 * TILE_SIZE,
        world.height() as f32 * TILE_SIZE,
    );

    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, scene_size.x, scene_size.y));

    let mut accumulator = 0.0;

    loop {
        let frame_dt = get_frame_time();
        accumulator += frame_dt;

        while accumulator >= FIXED_DT {
            player.update(FIXED_DT);
            player.collide_with_world(&world);
            player.clamp_to_scene(scene_size);
            accumulator -= FIXED_DT;
        }

        camera.zoom = vec2(
            (2.0 / screen_width()) * CAMERA_ZOOM,
            (2.0 / screen_height()) * CAMERA_ZOOM,
        );

        clear_background(BLACK);

        let half_viewport = vec2(
            screen_width() / (2.0 * CAMERA_ZOOM),
            screen_height() / (2.0 * CAMERA_ZOOM),
        );
        let player_center = player.position + vec2(PLAYER_SIZE * 0.5, PLAYER_SIZE * 0.5);
        let min_target = half_viewport;
        let max_target = vec2(
            scene_size.x - half_viewport.x,
            scene_size.y - half_viewport.y,
        );

        let base_target = vec2(
            player_center.x.clamp(
                min_target.x.min(max_target.x),
                max_target.x.max(min_target.x),
            ),
            player_center.y.clamp(
                min_target.y.min(max_target.y),
                max_target.y.max(min_target.y),
            ),
        );
        let shake_strength = if player.is_moving() {
            CAMERA_SHAKE_AMOUNT
        } else {
            0.0
        };
        let t = get_time() as f32;
        let shake = vec2(
            (t * CAMERA_SHAKE_FREQ_X).sin(),
            (t * CAMERA_SHAKE_FREQ_Y).sin(),
        ) * shake_strength;

        camera.target = vec2(
            (base_target.x + shake.x).clamp(
                min_target.x.min(max_target.x),
                max_target.x.max(min_target.x),
            ),
            (base_target.y + shake.y).clamp(
                min_target.y.min(max_target.y),
                max_target.y.max(min_target.y),
            ),
        );
        set_camera(&camera);

        // 🔥 desenha mundo
        world.draw();
        player.draw();

        // volta pra UI (se precisar depois)
        set_default_camera();

        next_frame().await;
    }
}
