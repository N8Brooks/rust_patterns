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
