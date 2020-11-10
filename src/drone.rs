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
        Drone::<Hovering>::new_from_idle(self)
    }
}

impl Drone<Hovering> {
    fn new_from_idle(drone: Drone<Idle>) -> Self {
        Self {
            state: PhantomData,
            x: drone.x,
            y: drone.y,
        }
    }

    fn new_from_flying(drone: Drone<Flying>) -> Self {
        Self {
            state: PhantomData,
            x: drone.x,
            y: drone.y,
        }
    }

    fn land(self) -> Drone<Idle> {
        Drone::<Idle>::new()
    }

    // &self == self -> Self
    // using self as a reference is the same as consuming self to return it
    // however, if the state transitions inside the function,
    // we are required to consume and return
    fn move_to(self, x: f32, y: f32) -> Drone<Hovering> {
        let drone = Drone::<Flying>::new(self);
        drone.fly(x, y)
    }
}

impl Drone<Flying> {
    fn new(drone: Drone<Hovering>) -> Self {
        Self {
            state: PhantomData,
            x: drone.x,
            y: drone.y,
        }
    }

    fn hasArrived(&self, x: f32, y: f32) -> bool {
        return self.x == x && self.y == y;
    }

    fn fly(mut self, x: f32, y: f32) -> Drone<Hovering> {
        self.x = x;
        self.y = y;
        Drone::<Hovering>::new_from_flying(self)
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
