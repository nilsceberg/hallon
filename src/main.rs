mod render_target;
mod display_device;
mod math;

use display_device::DisplayDevice;

fn main() {
	let rt = render_target::RenderTarget::new(10, 10);
	let dd = display_device::ConsoleDisplay;

	dd.show(&rt);
}
