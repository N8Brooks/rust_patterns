mod pizza_store {
    use crate::pizzas::{MockPizza, Pizza, PizzaType};

    pub trait PizzaStore {
        fn order_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
            let mut pizza = self.create_pizza(pizza_type);
            pizza.prepare();
            pizza.bake();
            pizza.cut();
            pizza.package();
            pizza
        }

        fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza>;
    }

    pub struct MockPizzaStore;

    impl PizzaStore for MockPizzaStore {
        fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
            Box::new(MockPizza::default())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn order_pizza() {
            let pizza_store = MockPizzaStore;
            let pizza = pizza_store.order_pizza(PizzaType::Cheese);
            let status = pizza.get_status();
            assert!(status.is_prepared);
            assert!(status.is_baked);
            assert!(status.is_cut);
            assert!(status.is_packaged);
        }

        #[test]
        fn create_pizza() {
            let pizza_store = MockPizzaStore;
            let pizza = pizza_store.create_pizza(PizzaType::Cheese);
            let status = pizza.get_status();
            assert!(!status.is_prepared);
            assert!(!status.is_baked);
            assert!(!status.is_cut);
            assert!(!status.is_packaged);
        }
    }
}

mod pizzas {
    #[derive(Debug)]
    pub enum PizzaType {
        Cheese,
        Veggie,
        Clam,
        Pepperoni,
    }

    #[derive(Debug)]
    pub struct PizzaData {
        name: &'static str,
        dough: &'static str,
        sauce: &'static str,
        toppings: Vec<&'static str>,
        status: PizzaStatus,
    }

    #[derive(Debug, Default)]
    pub struct PizzaStatus {
        pub is_prepared: bool,
        pub is_baked: bool,
        pub is_cut: bool,
        pub is_packaged: bool,
    }

    pub trait Pizza {
        fn get_data(&self) -> &PizzaData;

        fn get_data_mut(&mut self) -> &mut PizzaData;

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
        fn get_data(&self) -> &PizzaData {
            &self.0
        }

        fn get_data_mut(&mut self) -> &mut PizzaData {
            &mut self.0
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
}

pub mod ny_pizza_store {
    use crate::{
        pizza_store::PizzaStore,
        pizzas::{Pizza, PizzaData},
    };

    pub struct NyStylePizzaStore;

    impl PizzaStore for NyStylePizzaStore {
        fn create_pizza(
            &self,
            pizza_type: crate::pizzas::PizzaType,
        ) -> Box<dyn crate::pizzas::Pizza> {
            todo!()
        }
    }

    pub struct NyStyleCheesePizza {
        data: PizzaData,
    }

    impl Default for NyStyleCheesePizza {
        fn default() -> Self {
            todo!()
        }
    }

    impl Pizza for NyStyleCheesePizza {
        fn get_data(&self) -> &PizzaData {
            &self.data
        }

        fn get_data_mut(&mut self) -> &mut PizzaData {
            &mut self.data
        }
    }
}
