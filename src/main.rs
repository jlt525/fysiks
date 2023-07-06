#[derive(Debug)]
struct Position(f64, f64, f64);
#[derive(Debug)]
struct Velocity(f64, f64, f64);
#[derive(Debug)]
struct Acceleration(f64, f64, f64);
#[derive(Debug)]
struct Momentum(f64, f64, f64);

#[derive(Debug)]
struct Object {
    mass: i32,
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
    momentum: Momentum,
}

impl Object {
    // v = u + at
    fn update_velocity(&mut self) {
        self.velocity.2 = self.velocity.2 + self.acceleration.2;
    }

    // p = mv
    fn update_momentum(&mut self) {
        self.momentum.2 = (self.mass as f64) * self.velocity.2;
    }

    // x = x + vt
    fn update_position(&mut self) {
        self.position.2 = self.position.2 + self.velocity.2 + (self.acceleration.2 / 2.0);
    }
}

fn main() {
    let mut frame: i32 = 0;
    let gravity: f64 = -9.81;
    let max_frames: i32 = 6;
    let max_down_velocity: f64 = -29.43;

    let mut particle = Object {
        mass: 9,
        position: Position(0.0, 0.0, 0.0),
        velocity: Velocity(0.0, 0.0, 0.0),
        acceleration: Acceleration(0.0, 0.0, 0.0),
        momentum: Momentum(0.0, 0.0, 0.0),
    };

    particle.acceleration.2 = gravity;

    // do something
    while frame <= max_frames {
        dbg!(&particle);
        //println!("frame: {}, velocity: {}, momentum: {}, z position: {}", frame, particle.velocity, particle.momentum, particle.position.2);

        particle.update_position();

        if particle.velocity.2 <= max_down_velocity {
            particle.acceleration.2 = 0.0;
        } else {
            particle.update_velocity();
        }

        particle.update_momentum();

        frame += 1;
    }
}


// variables:
// distance
// time
// velocity
// acceleration

// constants:
// gravity
// mass

// f = ma
// v = x/t
// s = vt + (a/2)(t^2)
