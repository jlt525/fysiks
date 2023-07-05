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
    momentum: Momentum,
}

impl Object {
    fn update_velocity(&mut self, acceleration: f64) {
        self.velocity.2 = self.velocity.2 + acceleration;
    }

    fn update_momentum(&mut self) {
        self.momentum.2 = (self.mass as f64) * self.velocity.2;
    }

    fn update_position(&mut self) {
        self.position.2 = self.position.2 + self.velocity.2
    }
}

fn main() {
    let mut frame: i32 = 0;
    let gravity: f64 = -9.81;
    let max_frames: i32 = 6;
    let max_down_velocity: f64 = -30.0;

    let mut particle = Object {
        mass: 9,
        position: Position(0.0, 0.0, 0.0),
        velocity: Velocity(0.0, 0.0, 0.0),
        momentum: Momentum(0.0, 0.0, 0.0),
    };

    // do something
    while frame <= max_frames {
        dbg!(&particle);
        //println!("frame: {}, velocity: {}, momentum: {}, z position: {}", frame, particle.velocity, particle.momentum, particle.position.2);

        if (particle.velocity.2 + gravity) > max_down_velocity {
            particle.update_velocity(gravity);
        }

        particle.update_momentum();
        particle.update_position();

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

// f=ma
// v=d/t
