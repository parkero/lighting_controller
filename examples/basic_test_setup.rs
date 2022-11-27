use embedded_time::rate::Extensions;
use lc::animations::{Animatable, Animation};
use lc::utility::default_translation_array;
use lc::{colors, default_animations, LightingController, LogicalStrip};
use lighting_controller as lc;

fn main() {
    let frame_rate = 60.Hz();
    let mut color_buffer: [colors::RGB8; 16] = [colors::C_OFF; 16];
    let ls = LogicalStrip::new(&mut color_buffer);
    let translation_array: [usize; 16] = default_translation_array(16);
    let animation =
        &mut Animation::new(default_animations::ANI_TEST, translation_array, frame_rate);
    let animation_array: [&mut dyn Animatable; 1] = [animation];
    let mut lc = LightingController::new(ls, animation_array, frame_rate);

    loop {
        //this should have a check to limit updates to match the frame_rate:
        lc.update();
    }
}
