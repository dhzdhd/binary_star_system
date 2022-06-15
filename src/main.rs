use glam::vec3;
use macroquad::{math, prelude::*, rand::gen_range};

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;
const GRAV_CONST: f32 = 6.67 * 10.;

#[derive(Clone, Copy)]
struct Star {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    radius: f32,
    mass: f32, // in 10^10 kgs
    color: Color,
}

impl Star {
    fn new(pos: Vec3, vel: Vec3, acc: Vec3, radius: f32, mass: f32) -> Star {
        Star {
            pos,
            vel,
            acc,
            radius,
            mass,
            color: YELLOW,
        }
    }

    fn update(&mut self, pos: Vec3, mass: f32) {
        let g: f32 = 6.67 * 10.0_f32.powf(-11.);

        let denom = ((pos.x - self.pos.x).powf(2.) + (pos.z - self.pos.z).powf(2.)).powf(1.5);
        self.acc.x = (g * mass * (pos.x - self.pos.x)) / denom;
        self.acc.z = (g * mass * (pos.z - self.pos.z)) / denom;

        self.vel += self.acc;
        self.pos += self.vel;
    }
}

fn get_com(vec: Vec<Star>) -> Vec3 {
    struct MassPos {
        mass: f32,
        pos: Vec3,
    }

    let com = vec
        .into_iter()
        .map(|star| MassPos {
            mass: star.mass,
            pos: star.pos,
        })
        .reduce(|accum, star| {
            let com_x = star.mass * star.pos.x + accum.mass * accum.pos.x;
            let com_z = star.mass * star.pos.z + accum.mass * accum.pos.z;
            let total_mass = star.mass + accum.mass;
            MassPos {
                mass: total_mass,
                pos: vec3(com_x / total_mass, 0., com_z / total_mass),
            }
        })
        .unwrap();

    com.pos
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Binary Star System"),
        window_width: 1260,
        window_height: 768,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

fn gen_random_vector(start: f32, end: f32) -> Vec3 {
    let get_rand = || gen_range(start, end);
    return vec3(get_rand(), get_rand(), get_rand());
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let mut star1 = Star::new(
        vec3(-10., 0., 0.),
        vec3(-0.1, 0., -0.1),
        vec3(0., 0., 0.),
        5.,
        10.0_f32.powf(10.),
    );
    let mut star2 = Star::new(
        vec3(10., 0., 0.),
        vec3(0.1, 0., 0.1),
        vec3(0., 0., 0.),
        2.5,
        9.0_f32.powf(10.),
    );
    get_com(Vec::from([star1, star2]));

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Space) {
            position.y += MOVE_SPEED;
        }
        if is_key_down(KeyCode::LeftControl) {
            position.y -= MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize()
            * 3.;

        right = front.cross(world_up).normalize() * 3.;
        up = right.cross(front).normalize() * 3.;

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(BLACK);

        // 3D
        set_camera(&Camera3D {
            position,
            up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(2000, 10., BLACK, GRAY);

        star1.update(star2.pos, star2.mass);
        star2.update(star1.pos, star1.mass);
        draw_sphere(star1.pos, star1.radius, None, star1.color);
        draw_sphere(star2.pos, star2.radius, None, star2.color);
        println!("{:?} {:?} {:?}", star1.acc, star1.vel, star1.pos);

        // Back to screen space, render some text
        set_default_camera();

        next_frame().await
    }
}
