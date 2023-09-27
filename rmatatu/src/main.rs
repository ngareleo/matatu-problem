use std::{ fmt::Display, time::Duration, thread, sync::{ Arc, Mutex } };
use inputbot::KeybdKey::{ WKey, SKey };

// I want to implement the problem with out a middle controller
#[derive(Debug)]
struct Matatu {
    speed: u8,
    acceleration: u8,
    deceleration: u8,
    limit: u8,
    is_moving: bool,
}

impl Display for Matatu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Matatu {
    pub fn new(speed: u8, acceleration: u8, deceleration: u8) -> Self {
        Self { speed, acceleration, deceleration, limit: 100, is_moving: false }
    }

    pub fn default() -> Self {
        Self { speed: 0, acceleration: 4, deceleration: 2, limit: 100, is_moving: false }
    }

    pub fn drive(&mut self) {
        loop {
            self.speedometer();
            match self.is_moving {
                true => self.accelerate(),
                false => self.decelerate(),
            }
            thread::sleep(Duration::from_secs(2));
        }
    }

    pub fn accelerate(&mut self) {
        self.speed += self.acceleration;
        if self.speed > self.limit {
            self.speed = self.limit;
        }
    }

    pub fn decelerate(&mut self) {
        if self.speed <= self.acceleration {
            self.speed = 0;
        } else {
            self.speed -= self.deceleration;
        }
    }

    pub fn speedometer(&self) {
        println!("Needle at {}km/h", self.speed)
    }
}

#[derive(Default, Debug)]
struct Conductor;

impl Conductor {
    fn start_matatu(&self, matatu: &mut Matatu) {
        println!("Tap! tap! Matatu is moving");
        if !matatu.is_moving {
            matatu.is_moving = true;
        }
    }

    fn stop_matatu(&self, matatu: &mut Matatu) {
        println!("Tap! tap! Matatu is stopping");
        if matatu.is_moving {
            matatu.is_moving = false;
        }
    }
}

fn main() {
    let m = Matatu::default();
    let m_arc = Arc::new(Mutex::new(m));
    {
        let m_copy = Arc::clone(&m_arc);
        let h = thread::spawn(move || {
            let mut t = m_copy.lock().unwrap();
            t.drive();
        });

        let i = thread::spawn(move || {
            let c = Conductor::default();
            let mut t = m_arc.lock().unwrap();
            WKey.bind(|| {
                c.start_matatu(&mut t);
            });

            SKey.bind(|| {
                c.stop_matatu(&mut t);
            });

            inputbot::handle_events();
        });

        i.join().unwrap();
        h.join().unwrap();
    }

    {}
}
