extern crate nannou;
use nannou::prelude::*;



fn main() {
    let width = 1200;
    let height = 600;
    nannou::app(model)
        .update(update)
        .size(width, height)
        .run();

}

struct Model {
    _window: window::Id,
}

struct Particle {
}

fn model(app: &App) -> Model {
    // Init stuff, gui etc...
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(GHOSTWHITE);

    // let rect2:Rect<f32> = Rect::from_w_h(50.0,50.0);
    // let mouse_pos = app.mouse.position();

    let radius = 100.0;
    // let mod2 = random_range(0.9, 1.1);
    let mod2 = app.time.sin();
    // println!(mod2)
    let points = (0..360).map(|i| {
        // let modifier = random_range(0.9, 1.1);
        let modifier = mod2;
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        
        // let lerped = pt2(
        //     // x.lerp(x*modifier, ),
        //     // y.lerp(x*modifier,),
        // )
        // return (lerped, STEELBLUE)
        return (pt2(x*modifier,y*modifier), STEELBLUE); 
    });


    draw.polygon()
        .points_colored(points);

    // Draw frame
    draw.to_frame(app, &frame).unwrap();
}

fn point_at(from:Vec2, to:Vec2) -> f32 {
    let point = Vec2::new(to.x-from.x, to.y-from.y);
    return point.y.to_radians().atan2(point.x.to_radians());
}
