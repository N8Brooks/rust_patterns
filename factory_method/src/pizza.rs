#[derive(Debug)]
pub enum PizzaType {
    Cheese,
    Veggie,
    Clam,
    Pepperoni,
}

#[derive(Debug)]
pub struct PizzaData {
    pub name: &'static str,
    pub dough: &'static str,
    pub sauce: &'static str,
    pub toppings: Vec<&'static str>,
    pub status: PizzaStatus,
}

#[derive(Debug, Default)]
pub struct PizzaStatus {
    pub is_prepared: bool,
    pub is_baked: bool,
    pub is_cut: bool,
    pub is_packaged: bool,
}

pub trait Pizza {
    fn get_data_mut(&mut self) -> &mut PizzaData;

    fn get_data(&self) -> &PizzaData;

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
        self.get_data_mut().status.is_cut = true
    }

    fn package(&mut self) {
        self.get_data_mut().status.is_packaged = true
    }

    fn get_status(&self) -> &PizzaStatus {
        &self.get_data().status
    }
}

pub struct MockPizza(PizzaData);

impl Default for MockPizza {
    fn default() -> Self {
        MockPizza(PizzaData {
            name: "Mock Pizza",
            dough: "Mock Dough",
            sauce: "Mock Sauce",
            toppings: vec!["Mock Topping"],
            status: PizzaStatus::default(),
        })
    }
}

impl Pizza for MockPizza {
    fn get_data_mut(&mut self) -> &mut PizzaData {
        &mut self.0
    }

    fn get_data(&self) -> &PizzaData {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert!(!pizza.get_status().is_cut);
        pizza.cut();
        assert!(pizza.get_status().is_cut);
    }

    #[test]
    fn package() {
        let mut pizza = MockPizza::default();
        assert!(!pizza.get_status().is_packaged);
        pizza.package();
        assert!(pizza.get_status().is_packaged);
    }
}
