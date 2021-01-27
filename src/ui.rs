use graphics::{CharacterCache, Context, Rectangle, Text, Transformed};
use graphics::rectangle::rectangle_by_corners;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, GlyphCache};

pub trait Element {
    fn draw(&self, ctx: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache);
    fn update(&self);
    fn pressed(&self, x: f64, y: f64);
    fn released(&self, x: f64, y: f64);
}

pub struct Button {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub c: Color,
}

impl Element for Button {
    fn draw(&self, ctx: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        let r: Rectangle = Rectangle::new_round(self.c, 10.0);
        let rect = rectangle_by_corners(self.x, self.y, self.x + self.width, self.y + self.height);
        r.draw(rect, &ctx.draw_state, ctx.transform, graphics);
        let font_size = 16;
        let mut t: Text = Text::new(font_size as u32);
        t.color = [1.0, 1.0, 1.0, 1.0];
        let string_width = glyph_cache.width(font_size, &self.text).unwrap();
        t.draw(&self.text,
               glyph_cache,
               &ctx.draw_state,
               ctx.trans(self.x + self.width / 2.0 - string_width / 2.0,
                         self.y + font_size as f64 * 1.5 + 2.0).transform,
               graphics).unwrap();
    }

    fn update(&self) {

    }

    fn pressed(&self, x: f64, y: f64) {
        println!("button press {} {}", x, y);
    }

    fn released(&self, x: f64, y: f64) {
        println!("button release {} {}", x, y);
    }
}

impl Button {}