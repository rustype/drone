use std::marker::PhantomData;
pub trait DroneState: sealed::Sealed {}

pub struct Drone<'a, State>
where
    State: DroneState,
{
    inner: &'a mut InnerDrone,
    state: PhantomData<State>,
}

pub struct InnerDrone {
    x: f32,
    y: f32,
}

impl InnerDrone {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Idle;
impl DroneState for Idle {}
pub struct Hovering;
impl DroneState for Hovering {}
pub struct Flying;
impl DroneState for Flying {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Idle {}
    impl Sealed for super::Hovering {}
    impl Sealed for super::Flying {}
}

impl<'a> Drone<'a, Idle> {
    pub fn new(inner : &'a mut InnerDrone) -> Self {
        Self {
            inner,
            state: PhantomData,
        }
    }

    pub fn take_off(self) -> Drone<'a, Hovering> {
        Drone::<Hovering>::from(self)
    }
}

impl<'a> Drone<'a, Hovering> {
    fn land(self) -> Drone<'a, Idle> {
        Drone::<Idle>::from(self)
    }

    // &self == self -> Self
    // using self as a reference is the same as consuming self to return it
    // however, if the state transitions inside the function,
    // we are required to consume and return
    fn move_to(self, x: f32, y: f32) -> Drone<'a, Hovering> {
        let drone = Drone::<Flying>::from(self);
        drone.fly(x, y)
    }
}

// This implements a bidirectional conversion from Drone<Idle> to Drone<Hovering>
impl<'a> From<Drone<'a, Idle>> for Drone<'a, Hovering> {
    fn from(drone: Drone<'a, Idle>) -> Self {
        Self {
            inner: drone.inner,
            state: PhantomData,
        }
    }
}

impl<'a> From<Drone<'a, Hovering>> for Drone<'a, Idle> {
    fn from(drone: Drone<'a, Hovering>) -> Self {
        Self {
            inner: drone.inner,
            state: PhantomData,
        }
    }
}

impl<'a> From<Drone<'a, Hovering>> for Drone<'a, Flying> {
    fn from(drone: Drone<'a, Hovering>) -> Self {
        Self {
            inner: drone.inner,
            state: PhantomData,
        }
    }
}

impl<'a> From<Drone<'a, Flying>> for Drone<'a, Hovering> {
    fn from(drone: Drone<'a, Flying>) -> Self {
        Self {
            inner: drone.inner,
            state: PhantomData,
        }
    }
}

impl<'a> Drone<'a, Flying> {
    fn has_arrived(&self, x: f32, y: f32) -> bool {
        return self.inner.x == x && self.inner.y == y;
    }

    fn fly(mut self, x: f32, y: f32) -> Drone<'a, Hovering> {
        self.inner.x = x;
        self.inner.y = y;
        Drone::<Hovering>::from(self)
    }
}

#[cfg(test)]
mod drone_test {
    use super::*;
    #[test]
    fn drone_spawns_idle() {
        let mut inner = InnerDrone::new(0.0, 0.0);
        let drone = Drone::<Idle>::new(&mut inner);
        assert_eq!(drone.inner.x, 0.0);
        assert_eq!(drone.inner.y, 0.0);
    }

    #[test]
    fn drone_takes_off_n_lands() {
        let mut inner = InnerDrone::new(0.0, 0.0);
        let drone = Drone::<Idle>::new(&mut inner);
        let drone = drone.take_off();
        assert_eq!(drone.inner.x, 0.0);
        assert_eq!(drone.inner.y, 0.0);
        drone.land();
    }

    #[test]
    fn drone_flies() {
        let mut inner = InnerDrone::new(0.0, 0.0);
        let drone = Drone::<Idle>::new(&mut inner).take_off().move_to(-5.0, -5.0).land();
        assert_eq!(drone.inner.x, -5.0);
        assert_eq!(drone.inner.y, -5.0);
    }

    #[test]
    fn drone_does_not_fly_idle() {
        let mut inner = InnerDrone::new(0.0, 0.0);
        let drone = Drone::<Idle>::new(&mut inner);
        // drone.move_to(10.0, 10.0); // comptime error: "move_to" is not a member of type Idle
        assert_eq!(drone.inner.x, 0.0);
        assert_eq!(drone.inner.y, 0.0);
    }
}

// struct NotDroneState;

// impl Drone<NotDroneState> {} // NotDroneState does not satisfy trait DroneState
