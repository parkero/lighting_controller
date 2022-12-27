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
	let ls = LogicalStrip::new(&mut [BLACK; 16]);
	let hc = hardware_controller(get pin references, framerate, etc.);
	let lc = lighting_controller(framerate, etc.);
	let a1 = lc::animations::<LED_COUNT>::new(params) //creates default translation array starting at 0
		.set_translation_array([array_values; size])
        .more_builders(with_params);
    let a2 = lc::animations::<LED_COUNT>::new(params) //creates default translation array starting at 0
          .set_translation_array([array_values; size])
          .more_builders(with_params);

	loop {
        // the hc.update() will be in charge of framerate limiting in addition to any platform specific items needed to be updated regularly
		if hc.update() {
            // This lets you manipulate animations as needed between updates
			lc.update(&mut ls, &mut [a1, a2]);
			hc.write(&ls.iter()); //use SmartLeds Trait Write Function
		}
	}
}
```
