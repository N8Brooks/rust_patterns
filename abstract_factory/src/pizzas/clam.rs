use crate::ingredients;

use super::pizza::*;

#[derive(Debug, Default)]
pub struct Clam(Data);

impl Pizza for Clam {
    fn prepare(&mut self, ingredient_factory: &dyn ingredients::factory::Factory) {
        let mut data = self.get_data_mut();
        data.ingredients.dough = Some(ingredient_factory.create_dough());
        data.ingredients.sauce = Some(ingredient_factory.create_sauce());
        data.ingredients.cheese = Some(ingredient_factory.create_cheese());
        data.ingredients.clam = Some(ingredient_factory.create_clam());
    }

    fn get_data_mut(&mut self) -> &mut Data {
        &mut self.0
    }

    fn get_data(&self) -> &Data {
        &self.0
    }

    fn get_variety(&self) -> Variety {
        Variety::Clam
    }
}

#[cfg(test)]
mod test {
    use crate::ingredients::factory::mock::MockIngredientFactory;

    use super::*;

    #[test]
    fn prepare() {
        let mut pizza = Clam::default();
        let ingredient_factory = Box::new(MockIngredientFactory);
        {
            let ingredients = pizza.get_ingredients();
            assert!(ingredients.dough.is_none());
            assert!(ingredients.sauce.is_none());
            assert!(ingredients.cheese.is_none());
            assert!(ingredients.veggies.is_empty());
            assert!(ingredients.pepperoni.is_none());
            assert!(ingredients.clam.is_none());
        }
        pizza.prepare(ingredient_factory.as_ref());
        {
            let ingredients = pizza.get_ingredients();
            assert!(ingredients.dough.is_some());
            assert!(ingredients.sauce.is_some());
            assert!(ingredients.cheese.is_some());
            assert!(ingredients.veggies.is_empty());
            assert!(ingredients.pepperoni.is_none());
            assert!(ingredients.clam.is_some());
        }
    }

    #[test]
    fn bake() {
        let mut pizza = Clam::default();
        assert!(!pizza.get_status().is_baked);
        pizza.bake();
        assert!(pizza.get_status().is_baked);
    }

    #[test]
    fn cut() {
        let mut pizza = Clam::default();
        assert_eq!(pizza.get_status().cut, None);
        pizza.cut();
        assert_eq!(pizza.get_status().cut, Some(Cut::Slice));
    }

    #[test]
    fn package() {
        let mut pizza = Clam::default();
        assert!(!pizza.get_status().is_packaged);
        pizza.package();
        assert!(pizza.get_status().is_packaged);
    }
}
