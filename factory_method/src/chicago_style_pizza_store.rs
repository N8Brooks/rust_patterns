use crate::{pizza, pizza_store::PizzaStore};

pub use crate::pizza::Pizza;

pub struct ChicagoStylePizzaStore;

impl PizzaStore for ChicagoStylePizzaStore {
    fn create_pizza(&self, pizza_type: pizza::Variety) -> Box<dyn pizza::Pizza> {
        match pizza_type {
            pizza::Variety::Cheese => Box::<ChicagoStyleCheesePizza>::default(),
            pizza::Variety::Veggie => unimplemented!(),
            pizza::Variety::Clam => unimplemented!(),
            pizza::Variety::Pepperoni => unimplemented!(),
        }
    }
}

struct ChicagoStyleCheesePizza {
    data: pizza::Data,
}

impl Default for ChicagoStyleCheesePizza {
    fn default() -> Self {
        ChicagoStyleCheesePizza {
            data: pizza::Data {
                variety: pizza::Variety::Cheese,
                name: "Chicago Style Deep Dish Cheese Pizza",
                dough: "Extra Thick Crust Dough",
                sauce: "Plum Tmoato Sauce",
                toppings: vec!["Shredded Mozzarella Cheese"],
                status: pizza::Status::default(),
            },
        }
    }
}

impl pizza::Pizza for ChicagoStyleCheesePizza {
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
        let pizza_store = ChicagoStylePizzaStore;
        let pizza = pizza_store.create_pizza(pizza::Variety::Cheese);
        assert_eq!(*pizza.get_variety(), pizza::Variety::Cheese);
    }

    #[test]
    fn chicago_style_cheese_pizza() {
        let pizza = ChicagoStyleCheesePizza::default();
        assert_eq!(*pizza.get_variety(), pizza::Variety::Cheese);
    }
}
