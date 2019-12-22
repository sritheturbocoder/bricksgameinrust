use bricks::brickui::{BrickUi, Drawable};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use super::Resettable;

pub struct Paddle {
    pub color: Color,
    pub initial_x: f32,
    pub initial_y: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub speed_multiplier: f32
}

impl Paddle {
    pub fn new(color: Color,
               initial_x: f32,
               initial_y: f32,
               x: f32,
               y: f32,
               width: f32,
               height: f32,
               speed: f32,
               speed_multiplier: f32) -> Paddle {

        let mut paddle = Paddle {
            color,
            initial_x,
            initial_y,
            x,
            y,
            width,
            height,
            speed,
            speed_multiplier: 1.0
        };

        paddle.reset();
        return paddle;
    }
}

impl Resettable for Paddle{

    fn reset(&mut self){
        self.x = self.initial_x;
        self.y = self.initial_y;
        self.speed_multiplier = 1.0;
    }
}

impl Drawable for Paddle {

    fn draw(&self, ui: &mut BrickUi) {
        ui.renderer.set_draw_color(self.color);
        ui.renderer.fill_rect(Rect::new_unwrap(self.x as i32,
                                    self.y as i32,
                                    self.width as u32,
                                    self.height as u32));
    }
}