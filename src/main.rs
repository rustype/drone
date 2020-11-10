use anyhow::{anyhow, Result};

fn main() {}

#[derive(Debug, PartialEq)]
pub(crate) enum DroneState {
    Idle,
    Hovering,
    Flying,
}

#[derive(Debug)]
pub struct Drone {
    pub(crate) state: DroneState,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Drone {
    // #[out(Idle)]
    pub fn new() -> Self {
        Self {
            state: DroneState::Idle,
            x: 0.0,
            y: 0.0,
        }
    }

    // #[in(Idle), out(Hovering)]
    pub fn take_off(&mut self) -> Result<()> {
        match self.state {
            DroneState::Idle => {
                self.state = DroneState::Hovering;
                Ok(())
            }
            _ => Err(anyhow!("drone is not idle")),
        }
    }

    // #[in=(Hovering, Flying), out=(Hovering)]
    pub fn move_to(&mut self, x: f32, y: f32) -> Result<()> {
        match self.state {
            DroneState::Idle => Err(anyhow!("drone has not taken off")),
            DroneState::Hovering | DroneState::Flying => {
                self.state = DroneState::Flying;
                self.fly(x, y);
                if self.has_arrived(x, y) {
                    self.state = DroneState::Hovering;
                }
                Ok(())
            }
        }
    }

    // #[in=(Flying)]
    pub fn has_arrived(&self, x: f32, y: f32) -> bool {
        self.x == x && self.y == y
    }

    // #[in=(Hovering, Flying)]
    fn fly(&mut self, x: f32, y: f32) {
        // for now, our drone teleports!
        self.x = x;
        self.y = y;
    }

    // #[in=(Hovering), out=(Flying)]
    pub fn land(&mut self) -> Result<()> {
        match self.state {
            DroneState::Flying => Err(anyhow!("drone is currently flying")),
            DroneState::Hovering => {
                self.state = DroneState::Idle;
                Ok(())
            }
            DroneState::Idle => Err(anyhow!("drone is idle")),
        }
    }
}

#[cfg(test)]
mod test_drone {
    use super::*;

    macro_rules! with_drone {
        ($drone_name:ident {$($es:expr;)+}) => {{
            let mut $drone_name = Drone::new();
            $($es);+
        }};
        ($drone_name:ident {$($es:expr;)+} $($drone_names:ident {$($exprs:expr;)+})+) => {{
            with_drone!($drone_name {$(($es);)+});
            with_drone!($($drone_names {$($exprs;)+})+);
        }};
    }

    #[test]
    fn drone_spawns_idle() {
        let drone = Drone::new();
        assert_eq!(drone.state, DroneState::Idle);
        assert_eq!(drone.x, 0.0);
        assert_eq!(drone.y, 0.0);
    }

    #[test]
    fn drone_takes_off_n_lands() {
        let mut drone = Drone::new();
        assert!(drone.take_off().is_ok());
        assert_eq!(drone.state, DroneState::Hovering);
        assert!(drone.land().is_ok());
        assert_eq!(drone.state, DroneState::Idle);
    }

    #[test]
    fn drone_flies() {
        let mut drone = Drone::new();
        assert!(drone.take_off().is_ok());
        assert!(drone.move_to(-5.0, -5.0).is_ok());
        assert!(drone.land().is_ok());
        assert_eq!(drone.x, -5.0);
        assert_eq!(drone.y, -5.0);
    }

    #[test]
    fn drone_does_not_fly_idle() {
        let mut drone = Drone::new();
        assert!(drone.move_to(10.0, 10.0).is_err()); // comptime error: "move_to" is not a member of type Idle
        assert_eq!(drone.x, 0.0);
        assert_eq!(drone.y, 0.0);
    }

    #[test]
    fn drone_does_not_land_idle() {
        with_drone! {
            drone {
                assert!(drone.land().is_err()); // comptime error: "land" is not a member of type Idle
                assert_eq!(drone.x, 0.0);
                assert_eq!(drone.y, 0.0);
            }
        }
    }

    #[test]
    fn drone_does_not_take_off_twice() {
        with_drone! {
            drone {
                assert!(drone.take_off().is_ok());
                assert!(drone.take_off().is_err()); // comptime error: "take_off" is not a member of type Hovering
            }
        }
    }
}
