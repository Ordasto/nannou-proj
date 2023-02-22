use nannou::{color::named, prelude::*};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    bg_color: String,
    x: f32,
    y: f32,
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model {
        bg_color: "honeydew".to_string(),
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.radius < 500.0 {
        model.radius += 1.0;
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background()
        .color(named::from_str(&model.bg_color).unwrap());
    draw.ellipse()
        .color(STEELBLUE)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}

