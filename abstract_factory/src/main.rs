use abstract_factory::{
    pizzas::{self, store::Store},
    regions,
};

fn main() {
    let pizza_store = regions::ny::PizzaStore::default();
    dbg!(&pizza_store.order_pizza(pizzas::Variety::Cheese));
    dbg!(&pizza_store.order_pizza(pizzas::Variety::Clam));
}
