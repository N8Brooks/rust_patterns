use std::fmt::Debug;

pub trait Beverage: Debug {
    fn get_description(&self) -> String;

    fn cost(&self) -> u64;
}

pub mod beverages {
    pub use dark_roast::DarkRoast;
    pub use decaf::Decaf;
    pub use espresso::Espresso;
    pub use house_blend::HouseBlend;
    pub use stub::Stub;

    pub mod stub {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct Stub {
            description: &'static str,
            cost: u64,
        }

        impl Stub {
            pub fn with_box() -> Box<dyn Beverage> {
                Box::new(Stub {
                    description: "Stub",
                    cost: 0,
                })
            }
        }

        impl Beverage for Stub {
            fn get_description(&self) -> String {
                String::from(self.description)
            }

            fn cost(&self) -> u64 {
                self.cost
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn get_description() {
                let beverage = Stub::with_box();
                assert_eq!(beverage.get_description(), "Stub");
            }

            #[test]
            fn get_cost() {
                let beverage = Stub::with_box();
                assert_eq!(beverage.cost(), 0);
            }
        }
    }

    pub mod espresso {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct Espresso;

        impl Espresso {
            pub fn with_box() -> Box<dyn Beverage> {
                Box::new(Espresso)
            }
        }

        impl Beverage for Espresso {
            fn get_description(&self) -> String {
                String::from("Espresso")
            }

            fn cost(&self) -> u64 {
                199
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn get_description() {
                let beverage = Espresso;
                assert_eq!(beverage.get_description(), "Espresso");
            }

            #[test]
            fn cost() {
                let beverage = Espresso;
                assert_eq!(beverage.cost(), 199);
            }
        }
    }

    pub mod house_blend {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct HouseBlend;

        impl HouseBlend {
            pub fn with_box() -> Box<dyn Beverage> {
                Box::new(HouseBlend)
            }
        }

        impl Beverage for HouseBlend {
            fn get_description(&self) -> String {
                String::from("House Blend")
            }

            fn cost(&self) -> u64 {
                89
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn get_description() {
                let beverage = HouseBlend::with_box();
                assert_eq!(beverage.get_description(), "House Blend");
            }

            #[test]
            fn cost() {
                let beverage = HouseBlend::with_box();
                assert_eq!(beverage.cost(), 89);
            }
        }
    }

    pub mod dark_roast {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct DarkRoast;

        impl DarkRoast {
            pub fn with_box() -> Box<dyn Beverage> {
                Box::new(DarkRoast)
            }
        }

        impl Beverage for DarkRoast {
            fn get_description(&self) -> String {
                String::from("Dark Roast")
            }

            fn cost(&self) -> u64 {
                99
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn get_description() {
                let beverage = DarkRoast::with_box();
                assert_eq!(beverage.get_description(), "Dark Roast");
            }

            #[test]
            fn cost() {
                let beverage = DarkRoast::with_box();
                assert_eq!(beverage.cost(), 99);
            }
        }
    }

    pub mod decaf {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct Decaf;

        impl Decaf {
            pub fn with_box() -> Box<dyn Beverage> {
                Box::new(Decaf)
            }
        }

        impl Beverage for Decaf {
            fn get_description(&self) -> String {
                String::from("Decaf")
            }

            fn cost(&self) -> u64 {
                105
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn get_description() {
                let beverage = Decaf;
                assert_eq!(beverage.get_description(), "Decaf");
            }

            #[test]
            fn cost() {
                let beverage = Decaf;
                assert_eq!(beverage.cost(), 105);
            }
        }
    }
}

pub mod condiments {
    pub use mocha::Mocha;
    pub use soy::Soy;
    pub use steamed_milk::SteamedMilk;
    pub use whip::Whip;

    pub mod mocha {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct Mocha(Box<dyn Beverage>);

        impl Mocha {
            pub fn with_box(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
                Box::new(Mocha(beverage))
            }
        }

        impl Beverage for Mocha {
            fn get_description(&self) -> String {
                self.0.get_description() + ", Mocha"
            }

            fn cost(&self) -> u64 {
                self.0.cost() + 20
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::beverages::stub::Stub;

            #[test]
            fn get_description() {
                let beverage = Stub::with_box();
                let beverage = Mocha::with_box(beverage);
                assert_eq!(beverage.get_description(), "Stub, Mocha");
            }

            #[test]
            fn cost() {
                let beverage = Stub::with_box();
                let beverage = Mocha::with_box(beverage);
                assert_eq!(beverage.cost(), 20);
            }
        }
    }

    pub mod soy {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct Soy(Box<dyn Beverage>);

        impl Soy {
            pub fn with_box(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
                Box::new(Soy(beverage))
            }
        }

        impl Beverage for Soy {
            fn get_description(&self) -> String {
                self.0.get_description() + ", Soy"
            }

            fn cost(&self) -> u64 {
                self.0.cost() + 15
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::beverages::stub::Stub;

            #[test]
            fn get_description() {
                let beverage = Stub::with_box();
                let beverage = Soy::with_box(beverage);
                assert_eq!(beverage.get_description(), "Stub, Soy");
            }

            #[test]
            fn cost() {
                let beverage = Stub::with_box();
                let beverage = Soy::with_box(beverage);
                assert_eq!(beverage.cost(), 15);
            }
        }
    }

    pub mod whip {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct Whip(Box<dyn Beverage>);

        impl Whip {
            pub fn with_box(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
                Box::new(Whip(beverage))
            }
        }

        impl Beverage for Whip {
            fn get_description(&self) -> String {
                self.0.get_description() + ", Whip"
            }

            fn cost(&self) -> u64 {
                self.0.cost() + 10
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::beverages::stub::Stub;

            #[test]
            fn get_description() {
                let beverage = Stub::with_box();
                let beverage = Whip::with_box(beverage);
                assert_eq!(beverage.get_description(), "Stub, Whip");
            }

            #[test]
            fn cost() {
                let beverage = Stub::with_box();
                let beverage = Whip::with_box(beverage);
                assert_eq!(beverage.cost(), 10);
            }
        }
    }

    pub mod steamed_milk {
        pub use crate::Beverage;

        #[derive(Debug)]
        pub struct SteamedMilk(Box<dyn Beverage>);

        impl SteamedMilk {
            pub fn with_box(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
                Box::new(SteamedMilk(beverage))
            }
        }

        impl Beverage for SteamedMilk {
            fn get_description(&self) -> String {
                self.0.get_description() + ", Steamed Milk"
            }

            fn cost(&self) -> u64 {
                self.0.cost() + 10
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::beverages::stub::Stub;

            #[test]
            fn get_description() {
                let beverage = Stub::with_box();
                let beverage = SteamedMilk::with_box(beverage);
                assert_eq!(beverage.get_description(), "Stub, Steamed Milk");
            }

            #[test]
            fn cost() {
                let beverage = Stub::with_box();
                let beverage = SteamedMilk::with_box(beverage);
                assert_eq!(beverage.cost(), 10);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::beverages::*;
    use crate::condiments::*;

    #[test]
    fn espresso_with_steamed_milk() {
        let mut beverage = Espresso::with_box();
        beverage = SteamedMilk::with_box(beverage);
        assert_eq!(beverage.get_description(), "Espresso, Steamed Milk");
        assert_eq!(beverage.cost(), 209);
    }

    #[test]
    fn double_mocha_dark_roast_with_whip() {
        let mut beverage = DarkRoast::with_box();
        beverage = Mocha::with_box(beverage);
        beverage = Mocha::with_box(beverage);
        beverage = Whip::with_box(beverage);
        assert_eq!(beverage.get_description(), "Dark Roast, Mocha, Mocha, Whip");
        assert_eq!(beverage.cost(), 149);
    }

    #[test]
    fn soy_mocha_house_blend_with_whip() {
        let mut beverage = HouseBlend::with_box();
        beverage = Soy::with_box(beverage);
        beverage = Mocha::with_box(beverage);
        beverage = Whip::with_box(beverage);
        assert_eq!(beverage.get_description(), "House Blend, Soy, Mocha, Whip");
        assert_eq!(beverage.cost(), 134);
    }

    #[test]
    fn decaf_with_a_lotta_whip() {
        let mut beverage = Decaf::with_box();
        beverage = Whip::with_box(beverage);
        beverage = Whip::with_box(beverage);
        beverage = Whip::with_box(beverage);
        assert_eq!(beverage.get_description(), "Decaf, Whip, Whip, Whip");
        assert_eq!(beverage.cost(), 135);
    }
}
