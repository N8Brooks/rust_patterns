use crate::pizza::{MockPizza, Pizza, PizzaType};

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
    fn create_pizza(&self, _pizza_type: PizzaType) -> Box<dyn Pizza> {
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
