// This file is for an example game.
// use the command below to imediatly compile your Rust code in this file and convert into Webassembly binary.
// cargo watch allows to manages it automatically for you.
// cargo watch -x 'build --example game --target wasm32-unknown-unknown' 
extern "C" {
	fn js_clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32);
}

pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
	unsafe {
		js_clear_screen_to_color(red, green, blue, alpha);
	}
}

fn main() {
	// Clear the screen to blue.
	RustyBucket::clear_screen_to_color(0.0, 0.0, 1.0, 1.0);
}