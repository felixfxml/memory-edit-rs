use graphics::{CharacterCache, Context, Rectangle, Text, Transformed};
use graphics::rectangle::rectangle_by_corners;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, GlyphCache};

pub trait Element {
    fn draw(&self, ctx: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache, x: f64, y: f64);
    fn update(&self);
    fn pressed(&self, x: f64, y: f64);
    fn released(&self, x: f64, y: f64);
    fn key_typed(&self, character: char, key_code: u32);
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_width(&self) -> f64;
    fn get_height(&self) -> f64;
    fn is_hovered(&self, x: f64, y: f64) -> bool {
        x >= self.get_x() && y >= self.get_y() && x <= self.get_x() + self.get_width() && y <= self.get_y() + self.get_height()
    }
}

pub struct Button {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub c: Color,
    pub function: fn(&Button)
}

impl Element for Button {
    fn draw(&self, ctx: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache, x: f64, y: f64) {
        let r: Rectangle = Rectangle::new_round(self.c, 10.0);
        let rect = rectangle_by_corners(self.x, self.y, self.x + self.width, self.y + self.height);
        r.draw(rect, &ctx.draw_state, ctx.transform, graphics);

        let font_size = 12;
        let mut t: Text = Text::new(font_size as u32);
        t.color = [1.0, 1.0, 1.0, 1.0];

        let hovered = self.is_hovered(x, y);
        if hovered {
            t.color = [0.8, 0.8, 0.8, 1.0];
        }

        let string_width = glyph_cache.width(font_size, &self.text).unwrap();
        t.draw(&self.text,
               glyph_cache,
               &ctx.draw_state,
               ctx.trans(self.x + self.width / 2.0 - string_width / 2.0,
                         self.y + self.height / 2.0 + font_size as f64 / 2.0).transform,
               graphics).unwrap();
    }

    fn update(&self) {}

    fn pressed(&self, _x: f64, _y: f64) {}

    fn released(&self, _x: f64, _y: f64) {}

    fn key_typed(&self, _character: char, _key_code: u32) {
        unimplemented!()
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_height(&self) -> f64 {
        self.height
    }
}

impl Button {
    pub fn action(&self) {
        (self.function)(self);
    }
}