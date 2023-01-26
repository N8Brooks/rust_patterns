pub mod duck {
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
}

pub mod fly_behavior {
    pub trait FlyBehavior {
        fn fly(&self) -> &'static str;
    }

    #[derive(Default)]
    pub struct FlyWithWings;

    impl FlyBehavior for FlyWithWings {
        fn fly(&self) -> &'static str {
            "I'm flying!"
        }
    }

    #[derive(Default)]
    pub struct FlyNoWay;

    impl FlyBehavior for FlyNoWay {
        fn fly(&self) -> &'static str {
            "I can't fly"
        }
    }

    #[derive(Default)]
    pub struct FlyRocketPowered;

    impl FlyBehavior for FlyRocketPowered {
        fn fly(&self) -> &'static str {
            "I'm flying with a rocket!"
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fly_with_wings() {
            let fly_behavior = FlyWithWings::default();
            assert_eq!(fly_behavior.fly(), "I'm flying!");
        }

        #[test]
        fn fly_no_way() {
            let fly_behavior = FlyNoWay::default();
            assert_eq!(fly_behavior.fly(), "I can't fly");
        }

        #[test]
        fn fly_rocket_powered() {
            let fly_behavior = FlyRocketPowered::default();
            assert_eq!(fly_behavior.fly(), "I'm flying with a rocket!");
        }
    }
}

pub mod quack_behavior {
    pub trait QuackBehavior {
        fn quack(&self) -> &'static str;
    }

    #[derive(Default)]
    pub struct Quack;

    impl QuackBehavior for Quack {
        fn quack(&self) -> &'static str {
            "Quack"
        }
    }

    #[derive(Default)]
    pub struct MuteQuack;

    impl QuackBehavior for MuteQuack {
        fn quack(&self) -> &'static str {
            "<< Silence >>"
        }
    }

    #[derive(Default)]
    pub struct Squeak;

    impl QuackBehavior for Squeak {
        fn quack(&self) -> &'static str {
            "Squeak"
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn quack() {
            let quack_behavior = Quack::default();
            assert_eq!(quack_behavior.quack(), "Quack")
        }

        #[test]
        fn mute_quack() {
            let quack_behavior = MuteQuack::default();
            assert_eq!(quack_behavior.quack(), "<< Silence >>")
        }

        #[test]
        fn squeak() {
            let quack_behavior = Squeak::default();
            assert_eq!(quack_behavior.quack(), "Squeak")
        }
    }
}
