use super::fly_behavior::{FlyBehavior, FlyNoWay, FlyWithWings};
use super::quack_behavior::{Quack, QuackBehavior};
use std::fmt::Display;

pub trait Duck
where
    Self: Display,
    Self: Default,
{
    fn get_fly_behavior(&self) -> &dyn FlyBehavior;

    fn quack(&self) -> &'static str {
        self.get_quack_behavior().quack()
    }

    fn get_quack_behavior(&self) -> &dyn QuackBehavior;

    fn fly(&self) -> &'static str {
        self.get_fly_behavior().fly()
    }

    fn swim(&self) -> &'static str {
        "All ducks float, even decoys!"
    }
}

// This duck's behaviors are not replaceable
#[derive(Default)]
struct MallardDuck {
    fly_behavior: FlyWithWings,
    quack_behavior: Quack,
}

impl Duck for MallardDuck {
    fn get_fly_behavior(&self) -> &dyn FlyBehavior {
        &self.fly_behavior
    }

    fn get_quack_behavior(&self) -> &dyn QuackBehavior {
        &self.quack_behavior
    }
}

impl Display for MallardDuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I'm a real Mallard duck")
    }
}

// This duck's behaviors are replaceable
struct ModelDuck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}

#[allow(clippy::derivable_impls)]
impl Default for ModelDuck {
    fn default() -> Self {
        ModelDuck {
            fly_behavior: Box::<FlyNoWay>::default(),
            quack_behavior: Box::<Quack>::default(),
        }
    }
}

impl Duck for ModelDuck {
    fn get_fly_behavior(&self) -> &dyn FlyBehavior {
        self.fly_behavior.as_ref()
    }

    fn get_quack_behavior(&self) -> &dyn QuackBehavior {
        self.quack_behavior.as_ref()
    }
}

impl Display for ModelDuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I'm a model duck")
    }
}

#[cfg(test)]
mod tests {
    use super::super::fly_behavior::FlyRocketPowered;
    use super::*;

    #[test]
    fn mallard_duck() {
        let duck = MallardDuck::default();
        assert_eq!(duck.fly(), "I'm flying!");
        assert_eq!(duck.quack(), "Quack");
        assert_eq!(duck.swim(), "All ducks float, even decoys!");
        assert_eq!(duck.to_string(), "I'm a real Mallard duck");
    }

    #[test]
    fn model_duck() {
        let duck = ModelDuck::default();
        assert_eq!(duck.fly(), "I can't fly");
        assert_eq!(duck.quack(), "Quack");
        assert_eq!(duck.swim(), "All ducks float, even decoys!");
        assert_eq!(duck.to_string(), "I'm a model duck");
    }

    #[test]
    fn replaceable_behaviors() {
        let mut duck = ModelDuck::default();
        assert_eq!(duck.fly(), "I can't fly");
        duck.fly_behavior = Box::new(FlyRocketPowered);
        assert_eq!(duck.fly(), "I'm flying with a rocket!");
    }
}

pub mod quack_behavior {}
