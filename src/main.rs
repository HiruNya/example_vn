extern crate svnf;
extern crate piston_window;
extern crate gfx_glyph;

use std::sync::Arc;
use svnf::{
    game::*,
    util::GfxGlyph,
};
use piston_window::{
    PistonWindow,
    WindowSettings,
    TextureSettings,
    Texture,
    Flip,
};
use gfx_glyph::GlyphBrushBuilder;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 560;

fn main() {
        let mut window: PistonWindow = WindowSettings::new("Hello, World!", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .expect("Error in creating the window!");
        let mut game = Game::new([WIDTH as f64, HEIGHT as f64]);
        let background = Arc::new(Texture::from_path(
            &mut window.factory,
            "../assets/megumin.jpg",
            Flip::None,
            &TextureSettings::new())
            .expect("Error in creating the background texture!"));
        game.add_background("background1".to_string(),background.clone());
        game.load_characters_from_file("./assets/characters.toml", &mut window.factory)
            .expect("Error in loading the characters!");
        game.story.load_from_file("main".to_string(), "./assets/scripts/script.txt")
            .expect("Error in loading the script from the string");
        game.story.set_script("main", None);
        game.apply_grid(32, 32);
        game.load_gui_from_file("./assets/gui.toml")
            .expect("Error in loading the GUI file!");
        game.load_input_from_file("./assets/input.toml")
            .expect("Error in loading the Input file!");
        let font: &[u8] = include_bytes!("C:\\Users\\Hiruna\\Documents\\Projects\\Rust\\vn\\example_vn\\assets\\HanaMinA.ttf");
        let mut brush = GlyphBrushBuilder::using_font_bytes(font)
            .build(window.factory.clone());
        while let Some(event) = window.next() {
            game.handle_event(&event);
            use piston_window::{Event, Loop};
            if let Event::Loop(Loop::Render(_)) = event {
                game.draw_text(&mut brush);
                window.draw_2d_with_text( & event, &mut brush, | c, g |{
                    &game.draw(c, g);
                }).expect("Error in writing the text!");
            }
        }
}