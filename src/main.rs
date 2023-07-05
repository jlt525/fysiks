#[derive(Debug)]
struct Position(f64, f64, f64);

#[derive(Debug)]
struct Object {
    mass: i32,
    position: Position,
    velocity: f64,
    momentum: f64,
}

impl Object {
    fn update_velocity(&mut self, acceleration: f64) {
        self.velocity = self.velocity + (acceleration * 1.0);
    }

    fn update_momentum(&mut self) {
        self.momentum = (self.mass as f64) * self.velocity;
    }

    fn update_position(&mut self) {
        self.position.2 = self.position.2 + self.velocity
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
        velocity: 0.0,
        momentum: 0.0,
    };

    // do something
    while frame <= max_frames {
        dbg!(&particle);
        //println!("frame: {}, velocity: {}, momentum: {}, z position: {}", frame, particle.velocity, particle.momentum, particle.position.2);

        if (particle.velocity + gravity) > max_down_velocity {
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
