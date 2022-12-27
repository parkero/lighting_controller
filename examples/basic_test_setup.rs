use embedded_time::rate::Extensions;
use lc::animations::{Animatable, Animation};
use lc::{default_animations, LightingController, LogicalStrip};
use lighting_controller as lc;
use rgb::RGB8;
use smart_leds::colors::*;

fn main() {
    let frame_rate = 60.Hz();
    let mut color_buffer: [RGB8; 16] = [BLACK; 16];
    let mut ls = LogicalStrip::new(&mut color_buffer);
    let animation = &mut Animation::<16>::new(default_animations::ANI_TEST, frame_rate);
    let animation_array: [&mut dyn Animatable; 1] = [animation];
    let mut lc = LightingController::new(animation_array, frame_rate);

    loop {
        //this should have a check to limit updates to match the frame_rate:
        lc.update(&mut ls);
    }
}
