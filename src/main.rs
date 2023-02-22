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
    world: World,
}

struct World{
    particles:Vec<Particle>,
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
    fn draw(self: &Self, app: &App, frame:&Frame){
        let draw = app.draw();
        let points = (0..360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * self.radius;
            let y = radian.cos() * self.radius; 
            return (pt2(x+self.pos_x,y+self.pos_y), STEELBLUE); 
        });
        draw.polygon()
            .points_colored(points);

        draw.to_frame(app, frame).unwrap();    
    }
    
    fn update(self: &mut Self){
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
    }
    
}


fn model(app: &App) -> Model {
    // Init stuff, gui etc...
    let _window = app.new_window().view(view).build().unwrap();
    let particle_count = 100;
    let mut particles:Vec<Particle> = Vec::with_capacity(particle_count);
    (0..particle_count).for_each(|p| {
        particles.push(Particle{
            radius:10.0,
            pos_x: 0.0, // should change this when i get an internet connection lmao 
            pos_y: nannou::rand::random_range(0.0, app.window_rect().w()/2.0),
            vel_x: 5.0,
            vel_y: -5.0,
        });
    });
    let world = World{particles};
    Model { _window, world}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for p in _model.world.particles.iter_mut(){
        p.update();
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(GHOSTWHITE);

    // let rect2:Rect<f32> = Rect::from_w_h(50.0,50.0);
    // let mouse_pos = app.mouse.position();
    // let p = Particle{radius:10.0, pos_x:100.0, pos_y:100.0, vel_x:0.0, vel_y: 0.0};
    // p.draw(app);
    draw.to_frame(app, &frame).unwrap();
    _model.world.particles.iter().for_each(|p| p.draw(app, &frame));
    
}

fn point_at(from:Vec2, to:Vec2) -> f32 {
    let point = Vec2::new(to.x-from.x, to.y-from.y);
    return point.y.to_radians().atan2(point.x.to_radians());
}

fn intersect(p1:Particle, p2:Particle) -> bool{
    let a = p1.radius + p2.radius;
    let dx = p1.pos_x - p2.pos_x;
    let dy = p1.pos_y - p2.pos_y;
    return a * a > (dx*dx + dy*dy);
}
