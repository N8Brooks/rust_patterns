use crate::{ingredients, pizzas};

#[derive(Debug, Default)]
pub struct IngredientFactory;

impl ingredients::factory::Factory for IngredientFactory {
    fn create_dough(&self) -> Box<dyn ingredients::Dough> {
        Box::new(ThinCrustDough)
    }

    fn create_sauce(&self) -> Box<dyn ingredients::Sauce> {
        Box::new(MarinaraSauce)
    }

    fn create_cheese(&self) -> Box<dyn ingredients::Cheese> {
        Box::new(ReggianoCheese)
    }

    fn create_veggies(&self) -> Vec<Box<dyn ingredients::Veggie>> {
        vec![
            Box::new(Garlic),
            Box::new(Onion),
            Box::new(Mushroom),
            Box::new(RedPepper),
        ]
    }

    fn create_pepperoni(&self) -> Box<dyn ingredients::Pepperoni> {
        Box::new(SlicedPepperoni)
    }

    fn create_clam(&self) -> Box<dyn ingredients::Clam> {
        Box::new(FreshClam)
    }
}

#[derive(Debug)]
struct ThinCrustDough;

impl ingredients::Dough for ThinCrustDough {}

#[derive(Debug)]
struct MarinaraSauce;

impl ingredients::Sauce for MarinaraSauce {}

#[derive(Debug)]
struct ReggianoCheese;

impl ingredients::Cheese for ReggianoCheese {}

#[derive(Debug)]
struct Garlic;

impl ingredients::Veggie for Garlic {}

#[derive(Debug)]
struct Onion;

impl ingredients::Veggie for Onion {}

#[derive(Debug)]
struct Mushroom;

impl ingredients::Veggie for Mushroom {}

#[derive(Debug)]
struct RedPepper;

impl ingredients::Veggie for RedPepper {}

#[derive(Debug)]
struct SlicedPepperoni;

impl ingredients::Pepperoni for SlicedPepperoni {}

#[derive(Debug)]
struct FreshClam;

impl ingredients::Clam for FreshClam {}

#[derive(Debug, Default)]
pub struct PizzaStore(Box<IngredientFactory>);

impl pizzas::store::Store for PizzaStore {
    fn get_ingredient_factory(&self) -> &dyn ingredients::factory::Factory {
        self.0.as_ref()
    }
}
