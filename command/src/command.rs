use std::{cell::RefCell, rc::Rc};

pub trait Command {
    fn execute(&mut self);
}

#[derive(Default)]
pub struct Light(bool);

impl Light {
    pub fn is_on(&self) -> bool {
        self.0
    }

    pub fn is_off(&self) -> bool {
        !self.0
    }

    pub fn for_int_mut() -> Rc<RefCell<Light>> {
        Rc::new(RefCell::new(Light::default()))
    }
}

#[cfg(test)]
mod light_tests {
    use super::Light;

    #[test]
    fn is_on_true() {
        assert!(Light(true).is_on());
    }

    #[test]
    fn is_on_false() {
        assert!(!Light(false).is_on());
    }

    #[test]
    fn is_off_true() {
        assert!(Light(false).is_off());
    }

    #[test]
    fn is_off_false() {
        assert!(!Light(true).is_off());
    }
}

pub struct LightOn(Rc<RefCell<Light>>);

impl LightOn {
    pub fn new(light: &Rc<RefCell<Light>>) -> LightOn {
        LightOn(Rc::clone(&light))
    }
}

impl Command for LightOn {
    fn execute(&mut self) {
        self.0.borrow_mut().0 = true;
    }
}

#[cfg(test)]
mod light_on_tests {
    use super::*;

    #[test]
    fn execute() {
        let light = Light::for_int_mut();
        let mut command = LightOn::new(&light);
        assert!(light.borrow().is_off());
        command.execute();
        assert!(light.borrow().is_on());
    }
}
