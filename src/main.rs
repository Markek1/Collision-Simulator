use std::f32::consts::PI;

use macroquad::prelude::*;
use rand::Rng;
extern crate rand;

const WINDOW_SIZE: Vec2 = const_vec2!([1400., 800.]);
const G: Vec2 = const_vec2!([0., 0.]);

fn window_config() -> Conf {
    Conf {
        window_title: "Collision Simulator".to_owned(),
        window_width: WINDOW_SIZE.x.round() as i32,
        window_height: WINDOW_SIZE.y.round() as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, Copy, Debug)]
struct Ball {
    pos: Vec2,
    v: Vec2,
    r: f32,
    mass: f32,
    color: Color,
}

impl Ball {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.color);
    }

    fn update(&mut self, dt: f32, acc: Vec2) {
        self.v += acc * dt;

        if self.pos.x < self.r && self.v.x < 0.
            || WINDOW_SIZE.x - self.pos.x < self.r && self.v.x > 0.
        {
            self.v.x *= -1.;
        }
        if self.pos.y < self.r && self.v.y < 0.
            || WINDOW_SIZE.y - self.pos.y < self.r && self.v.y > 0.
        {
            self.v.y *= -1.;
        }
        self.pos += self.v;
    }

    fn check_collision(&self, other: &Ball) -> bool {
        other.pos.distance(self.pos) <= other.r + self.r
    }

    // Does collision effect for both self and the other object
    // Based on https://www.vobarian.com/collisions/2dcollisions2.pdf
    // The individual steps from the document are commented
    fn collide(&mut self, other: &mut Ball) {
        let pos_diff = self.pos - other.pos;

        // 1
        let unit_normal = pos_diff.normalize();
        let unit_tangent = Vec2::from((-unit_normal.y, unit_normal.x));

        // 3
        let v1n = self.v.dot(unit_normal);
        let v1t = self.v.dot(unit_tangent);
        let v2n = other.v.dot(unit_normal);
        let v2t = other.v.dot(unit_tangent);

        // 5
        let new_v1n =
            (v1n * (self.mass - other.mass) + 2. * other.mass * v2n) / (self.mass + other.mass);
        let new_v2n =
            (v2n * (other.mass - self.mass) + 2. * self.mass * v1n) / (self.mass + other.mass);

        // 6
        let final_v1n = new_v1n * unit_normal;
        let final_v1t = v1t * unit_tangent;
        let final_v2n = new_v2n * unit_normal;
        let final_v2t = v2t * unit_tangent;

        // 7
        let final_v1 = final_v1n + final_v1t;
        let final_v2 = final_v2n + final_v2t;

        // The if statement makes them not get stuck in each other
        if (final_v1 - final_v2).dot(self.pos - other.pos) > 0. {
            self.v = final_v1;
            other.v = final_v2;
        }
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut rng = rand::thread_rng();
    let mut paused = true;

    let n_balls = 100;
    let mut balls = Vec::with_capacity(n_balls);

    for i in 0..n_balls {
        let max_r = 6.;
        let r = rng.gen::<f32>() * max_r + 4.;
        balls.push(Ball {
            pos: Vec2::from((r * 2. + r * 2. * i as f32, r * 2. + r * i as f32)),
            v: Vec2::from((rng.gen::<f32>() * 4. - 2., rng.gen::<f32>() * 4. - 2.)),
            r: r,
            mass: PI * r.powf(2.),
            color: Color {
                r: rng.gen::<f32>() + 0.25,
                g: rng.gen::<f32>() + 0.25,
                b: rng.gen::<f32>() + 0.25,
                a: 1.,
            },
        })
    }

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if !paused {
            let dt = get_frame_time();

            for ball in balls.iter_mut() {
                ball.update(dt, G);
            }

            for i in 0..(balls.len() - 1) {
                let (left, right) = balls.split_at_mut(i + 1);
                for other_ball in right {
                    if left[i].check_collision(other_ball) {
                        left[i].collide(other_ball);
                    }
                }
            }
        }

        clear_background(BLACK);
        for ball in &balls {
            ball.draw();
        }

        next_frame().await
    }
}
