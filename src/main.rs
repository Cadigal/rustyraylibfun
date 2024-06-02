use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

// Converted from C++ to Rust
// https://github.com/Cadigal/raylibfun/blob/main/src/main.cpp
fn main() {
    // InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Window title");
    let (mut window, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Window title")
        .build();
    // SetTargetFPS(60);
    window.set_target_fps(60);

    // Texture2D texture = LoadTexture(ASSETS_PATH"test.png");
    let texture = window.load_texture(&thread, concat!("../../CLionProjects/raylibfun/assets/", "test.png")).unwrap();

    // while (!WindowShouldClose())
    while !window.window_should_close() {
        // BeginDrawing();
        let mut draw = window.begin_drawing(&thread);

        // ClearBackground(RAYWHITE);
        draw.clear_background(Color::RAYWHITE);

        // const int texture_x = SCREEN_WIDTH / 2 - texture.width / 2;
        // const int texture_y = SCREEN_HEIGHT / 2 - texture.height / 2;
        // DrawTexture(texture, texture_x, texture_y, WHITE);
        let texture_x = SCREEN_WIDTH / 2 - texture.width / 2;
        let texture_y = SCREEN_HEIGHT / 2 - texture.height / 2;
        draw.draw_texture(&texture, texture_x, texture_y, Color::WHITE);

        // const char* text = "OMG! IT WORKS!";
        // const Vector2 text_size = MeasureTextEx(GetFontDefault(), text, 20, 1);
        // DrawText(text, SCREEN_WIDTH / 2 - text_size.x / 2, texture_y + texture.height + text_size.y + 10, 20, BLACK);
        let text = "OMG! IT WORKS!";
        let text_size = draw.get_font_default().measure_text(&text, 20.0, 1.0);
        draw.draw_text(text, SCREEN_WIDTH / 2 - (text_size.x as i32) / 2, texture_y + texture.height + (text_size.y as i32) + 10, 20, Color::BLACK);

        // EndDrawing();
        // No explicit call needed, the drawing is ended when the draw variable goes out of scope
    }

    // CloseWindow();
    // No explicit call needed, the window is closed when the window variable goes out of scope
}
