extern crate piston_window;
extern crate piston;
extern crate num_traits;

use piston::event_loop::Events;
use piston_window::{WindowSettings, Loop, Event};

mod core;
mod tetris;

fn main() {
    let window = WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
    let mut app = tetris::TetrisApp::new(window);

    let mut event_loop = Events::new(app.create_event_settings());
    let mut input_events = Vec::with_capacity(50);

    while let Some(event) = event_loop.next(app.get_window()) {
        match event {
            // when we get input, hold on to it, and we'll let the app process it all at once on the next update frame
            Event::Input(input) => input_events.push(input),

            // most events are simple pass-throughs to the app
            Event::Loop(Loop::Render(args)) => app.render(&args),
            Event::Loop(Loop::AfterRender(args)) => app.after_render(&args),
            Event::Loop(Loop::Idle(args)) => app.idle(&args),
            
            // when we get an update event, send all the input we've accumulated this frame to the app when we call its update method
            Event::Loop(Loop::Update(args)) => {
                app.update(&args, input_events.as_slice());
                input_events.clear();
            },

            // todo: custom events
            other => println!("got unknown event: {:?}", other),
        }
    }
}