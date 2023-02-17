use crate::{pizza, pizza_store::PizzaStore};

pub struct NyStylePizzaStore;

impl PizzaStore for NyStylePizzaStore {
    fn create_pizza(&self, pizza_type: pizza::Variety) -> Box<dyn pizza::Pizza> {
        match pizza_type {
            pizza::Variety::Cheese => Box::<NyStyleCheesePizza>::default(),
            pizza::Variety::Veggie => unimplemented!(),
            pizza::Variety::Clam => unimplemented!(),
            pizza::Variety::Pepperoni => unimplemented!(),
        }
    }
}

struct NyStyleCheesePizza {
    data: pizza::Data,
}

impl Default for NyStyleCheesePizza {
    fn default() -> Self {
        NyStyleCheesePizza {
            data: pizza::Data {
                variety: pizza::Variety::Cheese,
                name: "NY Style Sauce and Cheese Pizza",
                dough: "Thin Crust Dough",
                sauce: "Marinara Sauce",
                toppings: vec!["Grated Reggiano Cheese"],
                status: pizza::Status::default(),
            },
        }
    }
}

impl pizza::Pizza for NyStyleCheesePizza {
    fn get_data_mut(&mut self) -> &mut pizza::Data {
        &mut self.data
    }

    fn get_data(&self) -> &pizza::Data {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_cheese_pizza() {
        let pizza_store = NyStylePizzaStore;
        let pizza = pizza_store.create_pizza(pizza::Variety::Cheese);
        assert_eq!(*pizza.get_variety(), pizza::Variety::Cheese);
    }
}
