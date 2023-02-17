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
