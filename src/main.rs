extern crate winapi;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::*;
use graphics::*;
use graphics::glyph_cache::rusttype::GlyphCache;
use opengl_graphics::*;
use piston::*;
use std::ops::Add;

mod process;

fn main() {
    process::run();
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Title", [500, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl_graphics: GlGraphics = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());

    let mut glyph_cache = GlyphCache::new("assets/JetBrainsMono-Regular.ttf", (), TextureSettings::new()).unwrap();

    while let Some(e) = events.next(&mut window) {
        if let Some(render) = e.render_args() {
            gl_graphics.draw(render.viewport(), |c, gl| {
                clear([0.2, 0.3, 0.4, 1.0], gl);
                for i in 0..5 {
                    let text: Text = Text::new_color([1.0, 1.0, 1.0, 1.0], 32);
                    text.draw(&*"Sample Text (".to_string().add(&*i.to_string()).add(")"),
                              &mut glyph_cache,
                              &c.draw_state,
                              c.trans(10.0, 100.0 + (i * 40) as f64).transform,
                              gl).unwrap();
                }
            });
        }
        /*
        if let Some(update) = e.update_args() {}
        if let Some(press) = e.press_args() {}
        if let Some(release) = e.release_args() {}
        */
    }
    //println!("{}", std::env::consts::OS);

    //println!("{}", &*"1".to_owned() + &*"2" + "3");


}
