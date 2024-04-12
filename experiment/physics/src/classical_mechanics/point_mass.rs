#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, PartialEq)]
pub struct Acceralation {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, PartialEq)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub struct PointMass {
    pub mass: f64,
    pub position: Position,
    pub velocity: Velocity,
    pub acceralation: Acceralation,
}

pub struct Force {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl PointMass {
    pub fn new(mass: Option<f64>, position: Option<Position>, velocity: Option<Velocity>, acceralation: Option<Acceralation>) -> Self {
        PointMass {
            mass: mass.unwrap_or(0.0),
            position: position.unwrap_or(Position {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }),
            velocity: velocity.unwrap_or(Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }),
            acceralation: acceralation.unwrap_or(Acceralation {
                x: 0.0,
                y: 0.0,
                z: 0.0
            })
        }
    }

    pub fn add_force(&mut self, f: Force) {
        self.acceralation = Acceralation {
            x: f.x / self.mass,
            y: f.y / self.mass,
            z: f.z / self.mass,
        };
    }

    pub fn velocity_after(&self, sec: f64) -> Velocity {
        Velocity {
            x: self.velocity.x + self.acceralation.x * sec,
            y: self.velocity.y + self.acceralation.y * sec,
            z: self.velocity.z + self.acceralation.z * sec,
        }
    }

    pub fn position_after(&self, sec: f64) -> Position {
        Position {
            x: self.position.x + self.velocity.x * sec + self.acceralation.x * (sec * sec) / 2.0,
            y: self.position.y + self.velocity.y * sec + self.acceralation.y * (sec * sec) / 2.0,
            z: self.position.z + self.velocity.z * sec + self.acceralation.z * (sec * sec) / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_force_test() {
        let mut body = PointMass::new(Some(1.0), None, None, None);

        body.add_force(Force {
            x: 0.0,
            y: 0.0,
            z: -9.8 * 1.0,
        });

        assert_eq!(body.acceralation, Acceralation {
            x: 0.0,
            y: 0.0,
            z: -9.8,
        });
    }

    #[test]
    fn velocity_after_test() {
        let mut body = PointMass::new(Some(1.0), None, None, None);

        body.add_force(Force {
            x: 0.0,
            y: 0.0,
            z: -9.8 * 1.0,
        });

        assert_eq!(body.velocity_after(1.0), Velocity {
            x: 0.0,
            y: 0.0,
            z: -9.8,
        });
    }

    #[test]
    fn position_after_test() {
        let mut body = PointMass::new(Some(1.0), None, None, None);

        body.add_force(Force {
            x: 0.0,
            y: 0.0,
            z: -9.8 * 1.0,
        });

        assert_eq!(body.position_after(1.0), Position {
            x: 0.0,
            y: 0.0,
            z: -4.9,
        });
    }
}