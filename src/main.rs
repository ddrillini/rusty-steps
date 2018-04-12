#[macro_use]
// unsure what it is, https://github.com/rust-lang/book/issues/401
// seems that rust has macros and they need to be enabled on a per-file basis

// https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

	let mut closed = false;
	while !closed {

		// draw blue
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.finish().unwrap();

		// listing the events produced by application and waiting to be received
		events_loop.poll_events(|ev| {
			match ev {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::Closed => closed = true,
					_ => (),
				},
				_ => (),
			}
		});
	}

}


