pub mod background;
pub mod foreground;
pub mod trigger;

use crate::utility::{
    convert_ns_to_frames, default_translation_array, Progression, StatefulRainbow,
};
use embedded_time::rate::Hertz;
use rgb::RGB8;

/// Adjust MAX_NUM_* consts depending on RAM requirements:
pub(crate) const MAX_NUM_ACTIVE_TRIGGERS: usize = 10;

/// This is the maximum offset value for rotating animations. It's basically the supersampled
/// resolution of the animation over the entire translation_array of leds.
pub const MAX_OFFSET: u16 = u16::MAX;

/// Denotes the direction of animations, effects vary depending on animation modes:
#[derive(Copy, Clone)]
pub enum Direction {
    Positive,
    Stopped,
    Negative,
}

/// Denotes the direction rainbow colors are used, effects vary depending on animation modes:
#[derive(Copy, Clone)]
pub enum RainbowDir {
    Forward,
    Backward,
}

/// Denotes the main types of animations, e.g. Foreground, Background, or Trigger:
#[derive(Clone, Copy)]
pub enum AnimationType {
    Background,
    Foreground,
    Trigger,
}

/// This holds the parameters that define everything needed to set up an animation. It's a struct
/// holding the parameters for the foreground animation, the background animation, and the global
/// information for trigger animations (such as the trigger Rainbow)
pub struct AnimationParameters<'a> {
    pub bg: background::Parameters<'a>,
    pub fg: foreground::Parameters<'a>,
    pub trigger: trigger::GlobalParameters<'a>,
}

/// This struct contains all the fixed parameters of an animation, as well as the state of the
/// foreground, background, and active trigger animations. It is updated by the LightingController
/// that it is attached to at the LightingController's frame rate based on the parameters provided.
/// To make a new animation,
pub struct Animation<'a, const N_LED: usize> {
    translation_array: [usize; N_LED],
    segment: [RGB8; N_LED],
    fg_state: foreground::Foreground<'a>,
    bg_state: background::Background<'a>,
    triggers: trigger::TriggerCollection<'a, MAX_NUM_ACTIVE_TRIGGERS>,
}

pub trait Animatable<'a> {
    fn update(&mut self);
    fn set_offset(&mut self, a_type: AnimationType, offset: u16);
    fn trigger(&mut self, params: &trigger::Parameters, frame_rate: Hertz);
    fn segment(&self) -> &[RGB8];
    fn translation_array(&self) -> &[usize];

    fn update_translation_array(&mut self, new_array: &[usize]);

    fn update_bg_direction(&mut self, new_direction: Direction);
    fn update_bg_duration_ns(&mut self, new_time: u64, frame_rate: Hertz);
    fn update_bg_mode(&mut self, new_mode: background::Mode);
    fn update_bg_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir);
    fn update_bg_subdivisions(&mut self, new_value: usize);

    fn update_fg_direction(&mut self, new_direction: Direction);
    fn update_fg_duration_ns(&mut self, new_time: u64, frame_rate: Hertz);
    fn update_fg_mode(&mut self, new_mode: foreground::Mode);
    fn update_fg_pixels_per_pixel_group(&mut self, new_value: usize);
    fn update_fg_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir);
    fn update_fg_step_time_ns(&mut self, new_time: u64, frame_rate: Hertz);
    fn update_fg_subdivisions(&mut self, new_value: usize);

    fn update_trig_duration_ns(&mut self, new_time: u64, frame_rate: Hertz);
    fn update_trig_fade_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir);
    fn update_trig_incremental_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir);
}

impl<'a, const N_LED: usize> Animatable<'a> for Animation<'a, N_LED> {
    fn update(&mut self) {
        // Update all three states
        self.bg_state.update(&mut self.segment);
        self.fg_state.update(&mut self.segment);
        self.triggers.update(&mut self.segment);
    }

    fn set_offset(&mut self, a_type: AnimationType, offset: u16) {
        match a_type {
            AnimationType::Background => {
                self.bg_state.offset = offset;
            }
            AnimationType::Foreground => {
                self.fg_state.offset = offset;
            }
            AnimationType::Trigger => {
                // Triggers don't use offsets, so do nothing until they need to.
            }
        }
    }

    fn trigger(&mut self, params: &trigger::Parameters, frame_rate: Hertz) {
        match params.mode {
            trigger::Mode::NoTrigger => {}
            trigger::Mode::Background => {
                self.bg_state.has_been_triggered = true;
            }
            trigger::Mode::Foreground => {
                self.fg_state.has_been_triggered = true;
            }
            _ => self.triggers.add_trigger(params, frame_rate),
        }
    }

    fn segment(&self) -> &[RGB8] {
        &self.segment[..]
    }

    fn translation_array(&self) -> &[usize] {
        &self.translation_array[..]
    }

    // universal settings functions: apply to all animation types - bg, fg, and triggers:

    fn update_translation_array(&mut self, new_array: &[usize]) {
        let source = new_array.iter().copied();
        let dest = self.translation_array.iter_mut();
        for (source, dest) in source.zip(dest) {
            *dest = source;
        }
    }

    // bg settings functions - for setting the bg animation parameters:

    fn update_bg_direction(&mut self, new_direction: Direction) {
        self.bg_state.direction = new_direction;
    }

    fn update_bg_duration_ns(&mut self, new_time: u64, frame_rate: Hertz) {
        let frame_count = convert_ns_to_frames(new_time, frame_rate);
        self.bg_state.frames = Progression::new(frame_count);
    }

    fn update_bg_mode(&mut self, new_mode: background::Mode) {
        self.bg_state.updater = new_mode.get_updater();
    }

    fn update_bg_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir) {
        self.bg_state.rainbow = StatefulRainbow::new(new_rainbow, rainbow_dir);
    }

    fn update_bg_subdivisions(&mut self, new_value: usize) {
        self.bg_state.subdivisions = new_value;
    }

    // fg settings functions - for setting the fg animation parameters:

    fn update_fg_direction(&mut self, new_direction: Direction) {
        self.fg_state.direction = new_direction;
    }

    fn update_fg_duration_ns(&mut self, new_time: u64, frame_rate: Hertz) {
        let frame_count = convert_ns_to_frames(new_time, frame_rate);
        self.fg_state.frames = Progression::new(frame_count);
    }

    fn update_fg_mode(&mut self, new_mode: foreground::Mode) {
        self.fg_state.updater = new_mode.get_updater();
    }

    fn update_fg_pixels_per_pixel_group(&mut self, new_value: usize) {
        self.fg_state.pixels_per_pixel_group = new_value;
    }

    fn update_fg_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir) {
        self.fg_state.rainbow = StatefulRainbow::new(new_rainbow, rainbow_dir);
    }

    fn update_fg_step_time_ns(&mut self, new_time: u64, frame_rate: Hertz) {
        let frame_count = convert_ns_to_frames(new_time, frame_rate);
        self.fg_state.step_frames = Progression::new(frame_count);
    }

    fn update_fg_subdivisions(&mut self, new_value: usize) {
        self.fg_state.subdivisions = new_value;
    }

    // trigger settings functions - for setting the trigger animation parameters:

    fn update_trig_duration_ns(&mut self, new_time: u64, frame_rate: Hertz) {
        let frame_count = convert_ns_to_frames(new_time, frame_rate);
        self.triggers.frames = Progression::new(frame_count);
    }

    fn update_trig_fade_rainbow(&mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir) {
        self.triggers.fade_rainbow = StatefulRainbow::new(new_rainbow, rainbow_dir);
    }

    fn update_trig_incremental_rainbow(
        &mut self,
        new_rainbow: &'a [RGB8],
        rainbow_dir: RainbowDir,
    ) {
        self.triggers.incremental_rainbow = StatefulRainbow::new(new_rainbow, rainbow_dir);
    }
}

impl<'a, const N_LED: usize> Animation<'a, N_LED> {
    pub fn new(parameters: AnimationParameters<'a>, frame_rate: Hertz) -> Self {
        let translation_array = default_translation_array(0);
        let segment = [RGB8::default(); N_LED];
        let fg_state = foreground::Foreground::new(&parameters.fg, frame_rate);
        let bg_state = background::Background::new(&parameters.bg, frame_rate);
        let triggers = trigger::TriggerCollection::new(&parameters.trigger, frame_rate);

        Animation {
            translation_array,
            segment,
            fg_state,
            bg_state,
            triggers,
        }
    }

    // universal settings functions: apply to all animation types - bg, fg, and triggers:

    pub fn set_translation_array(mut self, new_array: [usize; N_LED]) -> Self {
        self.update_translation_array(&new_array);
        self
    }

    // bg settings functions - for setting the bg animation parameters:

    pub fn set_bg_direction(mut self, new_direction: Direction) -> Self {
        self.update_bg_direction(new_direction);
        self
    }

    pub fn set_bg_duration_ns(mut self, new_time: u64, frame_rate: Hertz) -> Self {
        self.update_bg_duration_ns(new_time, frame_rate);
        self
    }

    pub fn set_bg_mode(mut self, new_mode: background::Mode) -> Self {
        self.update_bg_mode(new_mode);
        self
    }

    pub fn set_bg_rainbow(mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir) -> Self {
        self.update_bg_rainbow(new_rainbow, rainbow_dir);
        self
    }

    pub fn set_bg_subdivisions(mut self, new_value: usize) -> Self {
        self.update_bg_subdivisions(new_value);
        self
    }

    // fg settings functions - for setting the fg animation parameters:

    pub fn set_fg_direction(mut self, new_direction: Direction) -> Self {
        self.update_fg_direction(new_direction);
        self
    }

    pub fn set_fg_duration_ns(mut self, new_time: u64, frame_rate: Hertz) -> Self {
        self.update_fg_duration_ns(new_time, frame_rate);
        self
    }

    pub fn set_fg_mode(mut self, new_mode: foreground::Mode) -> Self {
        self.update_fg_mode(new_mode);
        self
    }

    pub fn set_fg_rainbow(mut self, new_rainbow: &'a [RGB8], rainbow_dir: RainbowDir) -> Self {
        self.update_fg_rainbow(new_rainbow, rainbow_dir);
        self
    }

    pub fn set_fg_pixels_per_pixel_group(mut self, new_value: usize) -> Self {
        self.update_fg_pixels_per_pixel_group(new_value);
        self
    }

    pub fn set_fg_step_time_ns(mut self, new_time: u64, frame_rate: Hertz) -> Self {
        self.update_fg_step_time_ns(new_time, frame_rate);
        self
    }

    pub fn set_fg_subdivisions(mut self, new_value: usize) -> Self {
        self.update_fg_subdivisions(new_value);
        self
    }

    // trigger settings functions  - for setting the trigger animation parameters:

    pub fn set_trig_duration_ns(mut self, new_time: u64, frame_rate: Hertz) -> Self {
        self.update_trig_duration_ns(new_time, frame_rate);
        self
    }

    pub fn set_trig_fade_rainbow(
        mut self,
        new_rainbow: &'a [RGB8],
        rainbow_dir: RainbowDir,
    ) -> Self {
        self.update_trig_fade_rainbow(new_rainbow, rainbow_dir);
        self
    }

    pub fn set_trig_incremental_rainbow(
        mut self,
        new_rainbow: &'a [RGB8],
        rainbow_dir: RainbowDir,
    ) -> Self {
        self.update_trig_incremental_rainbow(new_rainbow, rainbow_dir);
        self
    }
}
