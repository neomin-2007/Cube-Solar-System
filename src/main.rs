use macroquad::{prelude::*, telemetry::frame};
use universe::*;

mod universe;

#[macroquad::main("Solar System")]
async fn main() {
    
    let mut perspective_distance = 0.0;

    let mut perspective_user_x = 0.0;
    let mut perspective_user_y = 0.0;

    let mut sun = Orb::new_cube(2.0);
    sun.translation_dist = 1.0;
    sun.rot_y = 70.0;
    sun.rot_x = 30.0;
    
    let mut earth = Orb::new_cube(2.0);
    earth.translation_dist = 12.0;
    earth.rot_y = 70.0;
    earth.rot_x = 30.0;
    
    let mut moon = Orb::new_cube(0.5);
    moon.translation_dist = 12.0;
    moon.rot_y = 70.0;
    moon.rot_x = 30.0;

    loop {
        clear_background(BLACK);

        draw_text("Perspective size: ", 525.0, 570.0, 24.0, WHITE);
        draw_text(&perspective_distance.to_string().to_owned(), 725.0, 570.0, 24.0, WHITE);

        if is_key_pressed(KeyCode::Minus) {
            perspective_distance -= 2.0;
        }
        if is_key_pressed(KeyCode::Equal) {
            perspective_distance += 2.0;
        }
        if is_key_down(KeyCode::Left) {
            perspective_user_x -= 2.0;
        }
        if is_key_down(KeyCode::Right) {
            perspective_user_x += 2.0;
        }
        if is_key_down(KeyCode::Up) {
            perspective_user_y -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            perspective_user_y += 2.0;
        }
        
        sun.size += perspective_distance;
        earth.size += perspective_distance;
        moon.size += perspective_distance;

        sun.rot_y += 2.5;
        Orb::render(&sun, 50.0 + perspective_user_x, -100.0 + perspective_user_y);

        moon.rot_y += 2.5;
        Orb::render(&moon, 50.0 + perspective_user_x, -175.0 + perspective_user_y);

        earth.rot_y += 2.5;
        Orb::render(&earth, 0.0 + perspective_user_x, -175.0 + perspective_user_y);

        next_frame().await;
    }
}
