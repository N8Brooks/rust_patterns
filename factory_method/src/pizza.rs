#[derive(Debug, PartialEq)]
pub enum Variety {
    Cheese,
    Veggie,
    Clam,
    Pepperoni,
}

#[derive(Debug)]
pub struct Data {
    pub variety: Variety,
    pub name: &'static str,
    pub dough: &'static str,
    pub sauce: &'static str,
    pub toppings: Vec<&'static str>,
    pub status: Status,
}

#[derive(Debug, Default)]
pub struct Status {
    pub is_prepared: bool,
    pub is_baked: bool,
    pub cut: Option<Cut>,
    pub is_packaged: bool,
}

#[derive(Debug, Default, PartialEq)]
pub enum Cut {
    #[default]
    Slice,
    Square,
    Spiral,
}

pub trait Pizza {
    fn get_data_mut(&mut self) -> &mut Data;

    fn get_data(&self) -> &Data;

    fn get_variety(&self) -> &Variety {
        &self.get_data().variety
    }

    fn get_name(&self) -> &str {
        self.get_data().name
    }

    fn prepare(&mut self) {
        self.get_data_mut().status.is_prepared = true
    }

    fn bake(&mut self) {
        self.get_data_mut().status.is_baked = true
    }

    fn cut(&mut self) {
        self.get_data_mut().status.cut = Some(Cut::Slice)
    }

    fn package(&mut self) {
        self.get_data_mut().status.is_packaged = true
    }

    fn get_status(&self) -> &Status {
        &self.get_data().status
    }
}

pub mod mock_pizza {
    use super::*;

    pub struct MockPizza(Data);

    impl Default for MockPizza {
        fn default() -> Self {
            MockPizza(Data {
                variety: Variety::Cheese,
                name: "Mock Pizza",
                dough: "Mock Dough",
                sauce: "Mock Sauce",
                toppings: vec!["Mock Topping"],
                status: Status::default(),
            })
        }
    }

    impl Pizza for MockPizza {
        fn get_data_mut(&mut self) -> &mut Data {
            &mut self.0
        }

        fn get_data(&self) -> &Data {
            &self.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::mock_pizza::MockPizza;
    use super::*;

    #[test]
    fn get_variety() {
        let pizza = MockPizza::default();
        assert_eq!(*pizza.get_variety(), Variety::Cheese);
    }

    #[test]
    fn get_name() {
        let pizza = MockPizza::default();
        assert_eq!(pizza.get_name(), pizza.get_data().name);
    }

    #[test]
    fn prepare() {
        let mut pizza = MockPizza::default();
        assert!(!pizza.get_status().is_prepared);
        pizza.prepare();
        assert!(pizza.get_status().is_prepared);
    }

    #[test]
    fn bake() {
        let mut pizza = MockPizza::default();
        assert!(!pizza.get_status().is_baked);
        pizza.bake();
        assert!(pizza.get_status().is_baked);
    }

    #[test]
    fn cut() {
        let mut pizza = MockPizza::default();
        assert_eq!(pizza.get_status().cut, None);
        pizza.cut();
        assert_eq!(pizza.get_status().cut, Some(Cut::Slice));
    }

    #[test]
    fn package() {
        let mut pizza = MockPizza::default();
        assert!(!pizza.get_status().is_packaged);
        pizza.package();
        assert!(pizza.get_status().is_packaged);
    }
}
