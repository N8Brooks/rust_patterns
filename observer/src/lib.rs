#[derive(Clone)]
pub struct Measurements {
    temperature: f64,
    humidity: f64,
    _pressure: f64,
}

pub mod subject {
    use super::observer::Observer;
    use super::Measurements;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub trait Subject {
        fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
        fn remove_observer(&mut self, observer: &dyn Observer);
        fn notify_observers(&mut self);
    }

    #[derive(Default)]
    pub struct WeatherData {
        observers: Vec<Rc<RefCell<dyn Observer>>>,
        measurements: Option<Measurements>,
    }

    impl WeatherData {
        fn measurements_changed(&mut self) {
            self.notify_observers();
        }

        pub fn set_measurements(&mut self, measurements: Measurements) {
            self.measurements = Some(measurements);
            self.measurements_changed();
        }
    }

    impl Subject for WeatherData {
        fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
            self.observers.push(observer);
        }

        fn remove_observer(&mut self, _observer_a: &dyn Observer) {
            todo!();
        }

        fn notify_observers(&mut self) {
            if let Some(measurements) = self.measurements.clone() {
                for observer in self.observers.iter_mut() {
                    observer.borrow_mut().update(&measurements);
                }
            } else {
                panic!("measurements have not been initialized");
            }
        }
    }
}

pub mod observer {
    use super::Measurements;

    pub trait Observer {
        fn update(&mut self, metrics: &Measurements);
    }

    pub trait LoggingElement {
        fn log_data(&mut self);
        fn get_logs(&self) -> &Vec<String>;
    }

    #[derive(Default)]
    pub struct CurrentConditionsDisplay {
        temperature: f64,
        humidity: f64,
        logs: Vec<String>,
    }

    impl Observer for CurrentConditionsDisplay {
        fn update(&mut self, metrics: &Measurements) {
            self.temperature = metrics.temperature;
            self.humidity = metrics.humidity;
            self.log_data();
        }
    }

    impl LoggingElement for CurrentConditionsDisplay {
        fn log_data(&mut self) {
            let log = format!(
                "Current conditions: {}F degrees and {}% humidity",
                self.temperature, self.humidity,
            );
            self.logs.push(log);
        }

        fn get_logs(&self) -> &Vec<String> {
            return &self.logs;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::observer::*;
    use super::subject::*;
    use super::Measurements;
    use std::cell::RefCell;
    use std::rc::Rc;

    const MEASUREMENTS: Measurements = Measurements {
        temperature: 72.3,
        humidity: 50.1,
        _pressure: 24.0,
    };

    #[test]
    fn update_observer() {
        let mut weather_data = WeatherData::default();
        let current_display_a = Rc::new(RefCell::new(CurrentConditionsDisplay::default()));
        let current_display_b = Rc::clone(&current_display_a);
        weather_data.register_observer(current_display_a);
        weather_data.set_measurements(MEASUREMENTS);
        assert_eq!(
            current_display_b.borrow().get_logs(),
            &vec!["Current conditions: 72.3F degrees and 50.1% humidity"]
        );
    }

    // #[test]
    // fn remove_observer() {
    //     let mut weather_data = WeatherData::default();
    //     let current_display_a = Rc::new(RefCell::new(CurrentConditionsDisplay::default()));
    //     let current_display_b = Rc::clone(&current_display_a);
    //     weather_data.register_observer(current_display_a);
    //     weather_data.set_measurements(MEASUREMENTS);
    //     assert_eq!(current_display_b.borrow().get_logs(), &Vec::<String>::new());
    // }
}
