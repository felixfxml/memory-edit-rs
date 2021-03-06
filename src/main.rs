extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate process_memory;
extern crate winapi;

use glutin_window::{GlutinWindow, OpenGL};
use graphics::clear;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::{Button, EventLoop, Events, EventSettings, MouseCursorEvent, MouseScrollEvent, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, WindowSettings};

use crate::ui::Element;

mod ui;

fn main() {
    use sysinfo::*;

    let mut system = sysinfo::System::new_all();
    /*
    // First we update all information of our system struct.
        system.refresh_all();

    // Now let's print every process' id and name:
        for (pid, proc_) in system.get_processes() {
            println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
        }*/


    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Title", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl_graphics: GlGraphics = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());

    events = events.ups(1);

    let glyph_cache =
        &mut GlyphCache::new("assets/Roboto-Regular.ttf",
                             (),
                             TextureSettings::new())
            .unwrap();

    let b = ui::Button {
        x: 200.0,
        y: 100.0,
        width: 200.0,
        height: 40.0,
        text: "Sample Button Text".to_string(),
        c: [0.05, 0.1, 0.075, 1.0],
        function: (|_b| {
            println!("\nbutton pressed\n");
        })
    };

    let mut mouse_pos = [0.0, 0.0];

    while let Some(e) = events.next(&mut window) {
        e.mouse_cursor(|m| {
            mouse_pos = m;
        });

        //println!("Mouse position: {} {}", mouse_pos[0], mouse_pos[1]);

        if let Some(render) = e.render_args() {
            gl_graphics.draw(render.viewport(), |c, gl| {
                clear([0.2, 0.3, 0.4, 1.0], gl);

                b.draw(c, gl, glyph_cache, mouse_pos[0], mouse_pos[1]);
            });
        }

        if let Some(_update) = e.update_args() {
            b.update();
            system.refresh_all();
        }
        //TODO: find mouse pos
        if let Some(press) = e.press_args() {
            match press {
                Button::Keyboard(key) => println!("Pressed keyboard key '{:?}'", key),
                Button::Mouse(button) => {
                    println!("Pressed mouse button '{:?}'", button);
                    if b.is_hovered(mouse_pos[0], mouse_pos[1]) {
                        b.action();
                    }
                }
                _ => {}
            }
        }
        if let Some(release) = e.release_args() {
            match release {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
                _ => {}
            }
        }
        if let Some(_scroll) = e.mouse_scroll_args() {}
    }
    //println!("{}", std::env::consts::OS);

    //println!("{}", &*"1".to_owned() + &*"2" + "3");
}
