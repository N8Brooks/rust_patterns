use crate::{ingredients, pizzas};

pub trait Store {
    fn get_ingredient_factory(&self) -> &dyn ingredients::factory::Factory;

    fn order_pizza(&self, variety: pizzas::Variety) -> Box<dyn pizzas::Pizza> {
        let mut pizza = self.create_pizza(variety);
        let ingredient_factory = self.get_ingredient_factory();
        pizza.prepare(ingredient_factory);
        pizza.bake();
        pizza.cut();
        pizza.package();
        pizza
    }

    fn create_pizza(&self, pizza_variety: pizzas::Variety) -> Box<dyn pizzas::Pizza> {
        match pizza_variety {
            pizzas::Variety::Cheese => Box::<pizzas::cheese::Cheese>::default(),
            pizzas::Variety::Veggie => unimplemented!(),
            pizzas::Variety::Clam => Box::<pizzas::clam::Clam>::default(),
            pizzas::Variety::Pepperoni => unimplemented!(),
        }
    }
}

pub mod mock_pizza_store {
    use crate::ingredients::factory::mock::MockIngredientFactory;

    use super::*;

    #[derive(Default)]
    pub struct MockPizzaStore(Box<MockIngredientFactory>);

    impl Store for MockPizzaStore {
        fn create_pizza(&self, _pizza_type: pizzas::Variety) -> Box<dyn pizzas::Pizza> {
            Box::<pizzas::cheese::Cheese>::default()
        }

        fn get_ingredient_factory(&self) -> &dyn ingredients::factory::Factory {
            self.0.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mock_pizza_store::MockPizzaStore;
    use super::*;

    #[test]
    fn order_pizza() {
        let pizza_store = MockPizzaStore::default();
        let pizza = pizza_store.order_pizza(pizzas::Variety::Cheese);
        let status = pizza.get_status();
        assert!(status.is_baked);
        assert_eq!(status.cut, Some(pizzas::Cut::Slice));
        assert!(status.is_packaged);
    }

    #[test]
    fn create_pizza() {
        let pizza_store = MockPizzaStore::default();
        let pizza = pizza_store.create_pizza(pizzas::Variety::Cheese);
        let status = pizza.get_status();
        assert!(!status.is_baked);
        assert_eq!(status.cut, None);
        assert!(!status.is_packaged);
    }
}
