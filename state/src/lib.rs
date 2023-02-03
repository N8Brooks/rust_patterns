use has_quarter_state::HasQuarterState;
use no_quarter_state::NoQuarterState;
use sold_out_state::SoldOutState;
use sold_state::SoldState;
use std::convert::From;
use std::fmt::Debug;

enum GumballMachineStateId {
    NoQuarterState,
    HasQuarterState,
    SoldOutState,
    SoldState,
}

#[derive(Debug)]
enum GumballMachineStateError {
    NoQuarterInserted,
    AlreadyHasQuarter,
    CrankHasNotBeenTurned,
    AlreadyTurnedCrank,
}

trait GumballMachineState: Debug {
    fn get_id(&self) -> GumballMachineStateId;

    fn get_count(&self) -> u32;

    fn set_count(&mut self, count: u32);

    fn insert_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn eject_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn release_ball(&mut self) {
        self.set_count(self.get_count() - 1);
    }
}

mod no_quarter_state {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct NoQuarterState {
        count: u32,
    }

    impl GumballMachineState for NoQuarterState {
        fn get_id(&self) -> GumballMachineStateId {
            GumballMachineStateId::NoQuarterState
        }

        fn get_count(&self) -> u32 {
            self.count
        }

        fn set_count(&mut self, count: u32) {
            self.count = count;
        }

        fn insert_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Ok(Box::new(HasQuarterState::from(self.clone())))
        }

        fn eject_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::NoQuarterInserted)
        }

        fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::NoQuarterInserted)
        }

        fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::NoQuarterInserted)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn insert_quarter() {
            let mut state = Box::new(NoQuarterState::default());
            let mut state = state.insert_quarter().unwrap();
            assert!(matches!(
                state.get_id(),
                GumballMachineStateId::HasQuarterState
            ));
            assert_eq!(state.get_count(), 0);
        }

        #[test]
        fn eject_quarter() {
            let mut state = Box::new(NoQuarterState::default());
            assert!(matches!(
                state.eject_quarter().unwrap_err(),
                GumballMachineStateError::NoQuarterInserted
            ));
        }

        #[test]
        fn turn_crank() {
            let mut state = Box::new(NoQuarterState::default());
            assert!(matches!(
                state.turn_crank().unwrap_err(),
                GumballMachineStateError::NoQuarterInserted
            ));
        }

        #[test]
        fn dispense() {
            let mut state = Box::new(NoQuarterState::default());
            assert!(matches!(
                state.dispense().unwrap_err(),
                GumballMachineStateError::NoQuarterInserted
            ));
        }
    }
}

mod has_quarter_state {
    use super::*;

    #[derive(Debug, Default)]
    pub struct HasQuarterState {
        count: u32,
    }

    impl GumballMachineState for HasQuarterState {
        fn get_id(&self) -> GumballMachineStateId {
            GumballMachineStateId::HasQuarterState
        }

        fn get_count(&self) -> u32 {
            self.count
        }

        fn set_count(&mut self, count: u32) {
            self.count = count;
        }
        fn insert_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::AlreadyHasQuarter)
        }

        fn eject_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Ok(Box::new(NoQuarterState::default()))
        }

        fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Ok(Box::new(SoldState::default()))
        }

        fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::CrankHasNotBeenTurned)
        }
    }

    impl From<NoQuarterState> for HasQuarterState {
        fn from(state: NoQuarterState) -> Self {
            HasQuarterState {
                count: state.get_count(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn insert_quarter() {
            let mut state = Box::new(HasQuarterState::default());
            assert!(matches!(
                state.insert_quarter().unwrap_err(),
                GumballMachineStateError::AlreadyHasQuarter,
            ));
        }

        #[test]
        fn eject_quarter() {
            let mut state = Box::new(HasQuarterState::default());
            assert!(matches!(
                state.eject_quarter().unwrap().get_id(),
                GumballMachineStateId::NoQuarterState,
            ));
        }

        #[test]
        fn turn_crank() {
            let mut state = Box::new(HasQuarterState::default());
            assert!(matches!(
                state.turn_crank().unwrap().get_id(),
                GumballMachineStateId::SoldState,
            ));
        }

        #[test]
        fn dispense() {
            let mut state = Box::new(HasQuarterState::default());
            assert!(matches!(
                state.dispense().unwrap_err(),
                GumballMachineStateError::CrankHasNotBeenTurned
            ));
        }
    }
}

pub mod sold_state {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SoldState {
        count: u32,
    }

    impl GumballMachineState for SoldState {
        fn get_id(&self) -> GumballMachineStateId {
            GumballMachineStateId::SoldState
        }
        fn get_count(&self) -> u32 {
            self.count
        }

        fn set_count(&mut self, count: u32) {
            self.count = count;
        }
        fn insert_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::AlreadyTurnedCrank)
        }

        fn eject_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::AlreadyTurnedCrank)
        }

        fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            Err(GumballMachineStateError::AlreadyTurnedCrank)
        }

        fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            self.release_ball();
            if self.get_count() > 0 {
                Ok(Box::new(NoQuarterState::default()))
            } else {
                Ok(Box::new(SoldOutState::default()))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn insert_quarter() {
            let mut state = Box::new(SoldState::default());
            assert!(matches!(
                state.insert_quarter().unwrap_err(),
                GumballMachineStateError::AlreadyTurnedCrank,
            ));
        }

        #[test]
        fn eject_quarter() {
            let mut state = Box::new(SoldState::default());
            assert!(matches!(
                state.eject_quarter().unwrap_err(),
                GumballMachineStateError::AlreadyTurnedCrank,
            ));
        }

        #[test]
        fn turn_crank() {
            let mut state = Box::new(SoldState::default());
            assert!(matches!(
                state.turn_crank().unwrap_err(),
                GumballMachineStateError::AlreadyTurnedCrank,
            ));
        }

        #[test]
        fn dispense_to_not_sold_out() {
            let mut state = Box::new(SoldState { count: 2 });
            assert!(matches!(
                state.dispense().unwrap().get_id(),
                GumballMachineStateId::NoQuarterState,
            ));
        }

        #[test]
        fn dispense_to_sold_out() {
            let mut state = Box::new(SoldState { count: 1 });
            assert!(matches!(
                state.dispense().unwrap().get_id(),
                GumballMachineStateId::SoldOutState,
            ));
        }
    }
}

pub mod sold_out_state {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SoldOutState {
        count: u32,
    }

    impl GumballMachineState for SoldOutState {
        fn get_id(&self) -> GumballMachineStateId {
            GumballMachineStateId::SoldOutState
        }

        fn get_count(&self) -> u32 {
            self.count
        }

        fn set_count(&mut self, count: u32) {
            self.count = count;
        }
        fn insert_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            todo!()
        }

        fn eject_quarter(
            &mut self,
        ) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            todo!()
        }

        fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            todo!()
        }

        fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}

