use std::marker::PhantomData;

pub struct Idle;
pub struct Hovering;
pub struct Flying;

pub struct Drone<State> {
    x: f32,
    y: f32,
    state: PhantomData<State>,
}

impl Drone<Idle> {
    pub fn new() -> Self {
        Self {
            state: PhantomData,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn take_off(self) -> Drone<Hovering> {
        Drone::<Hovering>::from(self)
    }
}

impl Drone<Hovering> {
    fn land(self) -> Drone<Idle> {
        Drone::<Idle>::new()
    }

    // &self == self -> Self
    // using self as a reference is the same as consuming self to return it
    // however, if the state transitions inside the function,
    // we are required to consume and return
    fn move_to(self, x: f32, y: f32) -> Drone<Hovering> {
        let drone = Drone::<Flying>::from(self);
        drone.fly(x, y)
    }
}

// This implements a bidirectional conversion from Drone<Idle> to Drone<Hovering>
impl From<Drone<Idle>> for Drone<Hovering> {
    fn from(drone: Drone<Idle>) -> Self {
        Self {
            x : drone.x,
            y: drone.y,
            state: PhantomData
        }
    }
}

impl From<Drone<Flying>> for Drone<Hovering> {
    fn from(drone: Drone<Flying>) -> Self {
        Self {
            x: drone.x,
            y: drone.y,
            state: PhantomData
        }
    }
}

impl Drone<Flying> {
    fn hasArrived(&self, x: f32, y: f32) -> bool {
        return self.x == x && self.y == y;
    }

    fn fly(mut self, x: f32, y: f32) -> Drone<Hovering> {
        self.x = x;
        self.y = y;
        Drone::<Hovering>::from(self)
    }
}

impl From<Drone<Hovering>> for Drone<Flying> {
    fn from(drone: Drone<Hovering>) -> Self {
        Self {
            x: drone.x,
            y: drone.y,
            state: PhantomData
        }
    }
}

#[cfg(test)]
mod drone_test {
    use super::*;
    #[test]
    fn drone_spawns_idle() {
        let drone = Drone::<Idle>::new();
        assert_eq!(drone.x, 0.0);
        assert_eq!(drone.y, 0.0);
    }

    #[test]
    fn drone_takes_off_n_lands() {
        let drone = Drone::<Idle>::new();
        let drone = drone.take_off();
        assert_eq!(drone.x, 0.0);
        assert_eq!(drone.y, 0.0);
        drone.land();
    }

    #[test]
    fn drone_flies() {
        let drone = Drone::<Idle>::new();
        let drone = drone.take_off();
        let drone = drone.move_to(-5.0, -5.0);
        let drone = drone.land();
        assert_eq!(drone.x, -5.0);
        assert_eq!(drone.y, -5.0);
    }

    #[test]
    fn drone_does_not_fly_idle() {
        let drone = Drone::<Idle>::new();
        // drone.move_to(10.0, 10.0).is_err(); // comptime error: "move_to" is not a member of type Idle
        assert_eq!(drone.x, 0.0);
        assert_eq!(drone.y, 0.0);
    }
}
