use crate::pizza;

pub trait PizzaStore {
    fn order_pizza(&self, pizza_type: pizza::Variety) -> Box<dyn pizza::Pizza> {
        let mut pizza = self.create_pizza(pizza_type);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.package();
        pizza
    }

    fn create_pizza(&self, pizza_type: pizza::Variety) -> Box<dyn pizza::Pizza>;
}

pub mod mock_pizza_store {
    use super::*;
    use crate::pizza::mock_pizza::MockPizza;

    pub struct MockPizzaStore;

    impl PizzaStore for MockPizzaStore {
        fn create_pizza(&self, _pizza_type: pizza::Variety) -> Box<dyn pizza::Pizza> {
            Box::<MockPizza>::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mock_pizza_store::MockPizzaStore;
    use super::*;

    #[test]
    fn order_pizza() {
        let pizza_store = MockPizzaStore;
        let pizza = pizza_store.order_pizza(pizza::Variety::Cheese);
        let status = pizza.get_status();
        assert!(status.is_prepared);
        assert!(status.is_baked);
        assert_eq!(status.cut, Some(pizza::Cut::Slice));
        assert!(status.is_packaged);
    }

    #[test]
    fn create_pizza() {
        let pizza_store = MockPizzaStore;
        let pizza = pizza_store.create_pizza(pizza::Variety::Cheese);
        let status = pizza.get_status();
        assert!(!status.is_prepared);
        assert!(!status.is_baked);
        assert_eq!(status.cut, None);
        assert!(!status.is_packaged);
    }
}
