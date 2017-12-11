extern crate piston_window;

use piston_window::{WindowSettings, RenderArgs, ButtonState, Key, Loop, Button, UpdateArgs, AdvancedWindow, Event, IdleArgs, ButtonEvent, AfterRenderArgs, PistonWindow, Input, rectangle, clear};

struct TetrisApp {
    window: PistonWindow,
    holding_space: bool,
    red: f32,
    green: f32,
}

fn main() {
    let mut app = TetrisApp {
        window: WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap(),
        holding_space: false,
        red: 1.0,
        green: 0.0,
    };


    while let Some(event) = app.window.next() {
        match event {
            Event::Loop(Loop::Render(args)) => event_render(args, &mut app),
            Event::Loop(Loop::AfterRender(args)) => event_after_render(args, &mut app),
            Event::Loop(Loop::Update(args)) => event_update(args, &mut app),
            Event::Loop(Loop::Idle(args)) => event_idle(args, &mut app),
            Event::Input(args) => event_input(args, &mut app),
            other => println!("got unknown event: {:?}", other),
        }
    }
}

fn event_render(event: RenderArgs, app: &mut TetrisApp) {
    let placation_event = Event::from(Loop::from(event));
    let red = app.red;
    let green = app.green;

    app.window.draw_2d(&placation_event, |context, graphics| {
        clear([1.0; 4], graphics);
        rectangle([red, green, 0.0, 1.0], // red
                    [0.0, 0.0, 100.0, 100.0],
                    context.transform,
                    graphics);
    });
}
fn event_after_render(_: AfterRenderArgs, _: &mut TetrisApp) {

}

fn clamp<T: PartialOrd + Copy>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn event_update(args: UpdateArgs, app: &mut TetrisApp) {

    let dt = args.dt as f32;
    if app.holding_space {
        app.red = clamp(app.red - dt, 0.0, 1.0);
        app.green = clamp(app.green + dt, 0.0, 1.0);
    } else {
        app.red = clamp(app.red + dt, 0.0, 1.0);
        app.green = clamp(app.green - dt, 0.0, 1.0);
    }
}
fn event_idle(_: IdleArgs, _: &mut TetrisApp) {

}
fn event_input(args: Input, app: &mut TetrisApp) {
    if let Input::Button(args) = args {
        if args.button == Button::Keyboard(Key::Space) {
            app.holding_space = args.state == ButtonState::Press;
        }
    }
}