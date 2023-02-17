use std::fmt::Debug;

use crate::ingredients;

#[derive(Debug, PartialEq)]
pub enum Variety {
    Cheese,
    Veggie,
    Clam,
    Pepperoni,
}

#[derive(Debug, Default)]
pub struct Data {
    pub name: &'static str,
    pub status: Status,
    pub ingredients: ingredients::Ingredients,
}

#[derive(Debug, Default)]
pub struct Status {
    pub is_baked: bool,
    pub cut: Option<Cut>,
    pub is_packaged: bool,
}

#[derive(Debug, Default, PartialEq)]
pub enum Cut {
    #[default]
    Slice,
    Square,
    Spiral,
}

pub trait Pizza: Debug {
    fn get_data_mut(&mut self) -> &mut Data;

    fn get_data(&self) -> &Data;

    fn get_variety(&self) -> Variety;

    fn get_status(&self) -> &Status {
        &self.get_data().status
    }

    fn get_ingredients(&self) -> &ingredients::Ingredients {
        &self.get_data().ingredients
    }

    fn prepare(&mut self, ingredient_factory: &dyn ingredients::factory::Factory);

    fn bake(&mut self) {
        self.get_data_mut().status.is_baked = true
    }

    fn cut(&mut self) {
        self.get_data_mut().status.cut = Some(Cut::Slice)
    }

    fn package(&mut self) {
        self.get_data_mut().status.is_packaged = true
    }
}
