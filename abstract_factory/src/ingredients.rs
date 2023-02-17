use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct Ingredients {
    pub dough: Option<Box<dyn Dough>>,
    pub sauce: Option<Box<dyn Sauce>>,
    pub cheese: Option<Box<dyn Cheese>>,
    pub veggies: Vec<Box<dyn Veggie>>,
    pub pepperoni: Option<Box<dyn Pepperoni>>,
    pub clam: Option<Box<dyn Clam>>,
}

pub trait Dough: Debug {}

pub trait Sauce: Debug {}

pub trait Cheese: Debug {}

pub trait Veggie: Debug {}

pub trait Pepperoni: Debug {}

pub trait Clam: Debug {}

pub mod mock_ingredient {
    use super::*;

    #[derive(Debug, Default, PartialEq)]
    pub struct MockIngredient;

    impl Dough for MockIngredient {}

    impl Sauce for MockIngredient {}

    impl Cheese for MockIngredient {}

    impl Veggie for MockIngredient {}

    impl Pepperoni for MockIngredient {}

    impl Clam for MockIngredient {}
}

pub mod factory {
    use super::*;

    pub trait Factory {
        fn create_dough(&self) -> Box<dyn Dough>;
        fn create_sauce(&self) -> Box<dyn Sauce>;
        fn create_cheese(&self) -> Box<dyn Cheese>;
        fn create_veggies(&self) -> Vec<Box<dyn Veggie>>;
        fn create_pepperoni(&self) -> Box<dyn Pepperoni>;
        fn create_clam(&self) -> Box<dyn Clam>;
    }

    pub mod mock {
        use mock_ingredient::MockIngredient;

        use super::*;

        #[derive(Debug, Default)]
        pub struct MockIngredientFactory;

        impl Factory for MockIngredientFactory {
            fn create_dough(&self) -> Box<dyn Dough> {
                Box::new(MockIngredient)
            }

            fn create_sauce(&self) -> Box<dyn Sauce> {
                Box::new(MockIngredient)
            }

            fn create_cheese(&self) -> Box<dyn Cheese> {
                Box::new(MockIngredient)
            }

            fn create_veggies(&self) -> Vec<Box<dyn Veggie>> {
                Vec::new()
            }

            fn create_pepperoni(&self) -> Box<dyn Pepperoni> {
                Box::new(MockIngredient)
            }

            fn create_clam(&self) -> Box<dyn Clam> {
                Box::new(MockIngredient)
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            // Testing just to make sure there are no panics

            #[test]
            fn create_dough() {
                let ingredient_factory = MockIngredientFactory;
                let _ingredient = ingredient_factory.create_dough();
            }

            #[test]
            fn create_cheese() {
                let ingredient_factory = MockIngredientFactory;
                let _ingredient = ingredient_factory.create_cheese();
            }

            #[test]
            fn create_veggies() {
                let ingredient_factory = MockIngredientFactory;
                let _ingredient = ingredient_factory.create_veggies();
            }

            #[test]
            fn create_pepperoni() {
                let ingredient_factory = MockIngredientFactory;
                let _ingredient = ingredient_factory.create_pepperoni();
            }

            #[test]
            fn create_clam() {
                let ingredient_factory = MockIngredientFactory;
                let _ingredient = ingredient_factory.create_clam();
            }
        }
    }
}
