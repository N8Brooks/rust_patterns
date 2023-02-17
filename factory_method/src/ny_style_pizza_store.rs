use crate::{
    pizz_store::PizzaStore,
    pizza::{Pizza, PizzaData, PizzaStatus, PizzaType},
};

pub struct NyStylePizzaStore;

impl PizzaStore for NyStylePizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Cheese => Box::new(NyStyleCheesePizza::default()),
            PizzaType::Veggie => Box::new(NyStyleVeggiePizza::default()),
            PizzaType::Clam => Box::new(NyStyleClamPizza::default()),
            PizzaType::Pepperoni => Box::new(NyStylePepperoniPizza::default()),
        }
    }
}

struct NyStyleCheesePizza {
    data: PizzaData,
}

impl Default for NyStyleCheesePizza {
    fn default() -> Self {
        NyStyleCheesePizza {
            data: PizzaData {
                name: "NY Style Sauce and Cheese Pizza",
                dough: "Thin Crust Dough",
                sauce: "Marinara Sauce",
                toppings: vec!["Grated Reggiano Cheese"],
                status: PizzaStatus::default(),
            },
        }
    }
}

impl Pizza for NyStyleCheesePizza {
    fn get_data_mut(&mut self) -> &mut PizzaData {
        &mut self.data
    }

    fn get_data(&self) -> &PizzaData {
        &self.data
    }
}

struct NyStylePepperoniPizza {
    data: PizzaData,
}

impl Default for NyStylePepperoniPizza {
    fn default() -> Self {
        NyStylePepperoniPizza {
            data: PizzaData {
                name: "NY Style Sauce and Cheese Pizza",
                dough: "Thin Crust Dough",
                sauce: "Marinara Sauce",
                toppings: vec!["Grated Reggiano Cheese"],
                status: PizzaStatus::default(),
            },
        }
    }
}

impl Pizza for NyStylePepperoniPizza {
    fn get_data_mut(&mut self) -> &mut PizzaData {
        &mut self.data
    }

    fn get_data(&self) -> &PizzaData {
        &self.data
    }
}

struct NyStyleClamPizza {
    data: PizzaData,
}

impl Default for NyStyleClamPizza {
    fn default() -> Self {
        NyStyleClamPizza {
            data: PizzaData {
                name: "NY Style Sauce and Cheese Pizza",
                dough: "Thin Crust Dough",
                sauce: "Marinara Sauce",
                toppings: vec!["Grated Reggiano Cheese"],
                status: PizzaStatus::default(),
            },
        }
    }
}

impl Pizza for NyStyleClamPizza {
    fn get_data_mut(&mut self) -> &mut PizzaData {
        &mut self.data
    }

    fn get_data(&self) -> &PizzaData {
        &self.data
    }
}

struct NyStyleVeggiePizza {
    data: PizzaData,
}

impl Default for NyStyleVeggiePizza {
    fn default() -> Self {
        NyStyleVeggiePizza {
            data: PizzaData {
                name: "NY Style Sauce and Cheese Pizza",
                dough: "Thin Crust Dough",
                sauce: "Marinara Sauce",
                toppings: vec!["Grated Reggiano Cheese"],
                status: PizzaStatus::default(),
            },
        }
    }
}

impl Pizza for NyStyleVeggiePizza {
    fn get_data_mut(&mut self) -> &mut PizzaData {
        &mut self.data
    }

    fn get_data(&self) -> &PizzaData {
        &self.data
    }
}
