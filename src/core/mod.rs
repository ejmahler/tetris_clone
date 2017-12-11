
mod binary_axis;
pub use self::binary_axis::BinaryAxis;

use piston::event_loop::{Events, EventSettings};
use piston_window::{PistonWindow, RenderArgs, AfterRenderArgs, UpdateArgs, IdleArgs, Input, Event, Loop};

pub trait App {
    // config/setup methods
    fn create_event_settings(&self) -> EventSettings;
    fn get_window<'a>(&'a mut self) -> &'a mut PistonWindow;

    // event handler methods
    fn render(&mut self, args: &RenderArgs);
    fn after_render(&mut self, args: &AfterRenderArgs);
    fn update(&mut self, args: &UpdateArgs, input_events: &[Input]);
    fn idle(&mut self, args: &IdleArgs);
}

pub fn exec(app: &mut impl App) {
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