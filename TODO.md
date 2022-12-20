# Project Goals and Ideas
### Poorly formatted and thought out

### Major Goals, in nothing resembling order, for now.
- [ ] Support for RGB, RGBA, RGBW, and RGBWA led types. This should be compatible with the rgb crate's rgb color types.
- [ ] API update - lighting controller should use functions and builder paradigm to add animations. Animations should not need to be created separately outside the lighting controller.
  - Unsure if this should include the animation parameter structs as builder functions or not.
- [ ] Currently, the lighting controller takes ownership of an array of RGB8 values. refactor so it only operates on a reference to an array instead. The array should be passed into functions like lc.update(&array)
- [x] Lighting controller should be entirely platform-agnostic. All functions that use things like system clocks or sending color data to hardware pins should be relegated to a hardware controller instead.

### (extremely rough) example of how it could look when done:

```rust 
fn main() {
	let array = [colors go here];
	let hc = hardware_controller(get pin references);
	let lc = lighting_controller(framerate, etc.);
	lc.add_animation(size, params) //creates default translation array starting at 0
		.set_translation_array([array_values; size]);

	loop {
		if hc.update() {
			lc.update(&array);
			hc.write(&array); //use SmartLeds Trait Write Function
		}
	}
}
```
