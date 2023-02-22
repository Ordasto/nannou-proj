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
    radius:f32,
    pos_x:f32,
    pos_y:f32,
    vel_x:f32,
    vel_y:f32,
}

impl Particle {
    // note sure i need update tbh
    fn draw(self: &Self, app: &App){
        let draw = app.draw();
        let points = (0..360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * self.radius;
            let y = radian.cos() * self.radius; 
            return (pt2(x+self.pos_x,y+self.pos_y), STEELBLUE); 
        });

        draw.polygon()
            .points_colored(points);
        
    }
    
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
    let p = Particle{radius:10.0, pos_x:100.0, pos_y:100.0, vel_x:0.0, vel_y: 0.0};
    // Draw frame
    p.draw(app);
    draw.to_frame(app, &frame).unwrap();
}

fn point_at(from:Vec2, to:Vec2) -> f32 {
    let point = Vec2::new(to.x-from.x, to.y-from.y);
    return point.y.to_radians().atan2(point.x.to_radians());
}
