extern crate gobu;
extern crate piston_window;
extern crate gfx_glyph;

use gobu::{
    GameBuilder,
    util::GfxGlyph,
};
use piston_window::{
    PistonWindow,
    WindowSettings,
};
use std::{
    fs::File,
    io::Read,
};
use gfx_glyph::GlyphBrushBuilder;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 560;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello, World!", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .expect("Error in creating the window!");
    let mut game = GameBuilder::new([WIDTH as f64, HEIGHT as f64])
        .characters("./assets/config/characters.toml")
        .grid(32, 32)
        .gui("./assets/config/gui.toml")
        .input("./assets/config/input.toml")
        .backgrounds("./assets/config/background.toml")
        .scripts("./assets/config/scripts.toml")
        .music("./assets/config/music.toml")
        .build(&mut window.factory)
        .expect("Error in building the game!");
    let font = {
        let mut buffer = vec![];
        File::open("./assets/fonts/HanaMinA.ttf")
            .expect("Error finding font!")
            .read_to_end(&mut buffer)
            .expect("Error reading font!");
        buffer
    };
    let mut brush = GlyphBrushBuilder::using_font_bytes(font.as_slice())
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