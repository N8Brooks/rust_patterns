use crate::command;

pub struct SimpleRemoteControl {
    slot: Box<dyn command::Command>,
}

impl SimpleRemoteControl {
    pub fn new(command: Box<dyn command::Command>) -> SimpleRemoteControl {
        SimpleRemoteControl { slot: command }
    }

    pub fn set_command(&mut self, command: Box<dyn command::Command>) {
        self.slot = command;
    }

    pub fn button_was_pressed(&mut self) {
        self.slot.execute()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn set_command() {
        let light = command::Light::for_int_mut();
        let command = Box::new(command::LightOn::new(&light));
        let mut control = SimpleRemoteControl::new(command);
        assert_eq!(Rc::strong_count(&light), 2);
        {
            let light = command::Light::for_int_mut();
            let command = Box::new(command::LightOn::new(&light));
            control.set_command(command)
        }
        assert_eq!(Rc::strong_count(&light), 1);
    }

    #[test]
    fn button_was_pressed() {
        let light = command::Light::for_int_mut();
        let command = Box::new(command::LightOn::new(&light));
        let mut control = SimpleRemoteControl::new(command);
        assert!(light.borrow().is_off());
        control.button_was_pressed();
        assert!(light.borrow().is_on());
    }
}
