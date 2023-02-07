use crate::{
    no_quarter_state::NoQuarterState,
    sold_state::SoldState,
    state::{GumballMachineState, GumballMachineStateError, GumballMachineStateId},
};

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

    fn insert_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::AlreadyHasQuarter)
    }

    fn eject_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        let state: &dyn GumballMachineState = self;
        Ok(Box::new(NoQuarterState::from(state)))
    }

    fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        let state: &dyn GumballMachineState = self;
        Ok(Box::new(SoldState::from(state)))
    }

    fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::CrankHasNotBeenTurned)
    }
}

impl std::convert::From<&dyn GumballMachineState> for HasQuarterState {
    fn from(state: &dyn GumballMachineState) -> Self {
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
        assert_eq!(
            state.insert_quarter().unwrap_err(),
            GumballMachineStateError::AlreadyHasQuarter,
        );
    }

    #[test]
    fn eject_quarter() {
        let mut state = Box::new(HasQuarterState::default());
        assert_eq!(
            state.eject_quarter().unwrap().get_id(),
            GumballMachineStateId::NoQuarterState,
        );
    }

    #[test]
    fn turn_crank() {
        let mut state = Box::new(HasQuarterState::default());
        assert_eq!(
            state.turn_crank().unwrap().get_id(),
            GumballMachineStateId::SoldState,
        );
    }

    #[test]
    fn dispense() {
        let mut state = Box::new(HasQuarterState::default());
        assert_eq!(
            state.dispense().unwrap_err(),
            GumballMachineStateError::CrankHasNotBeenTurned
        );
    }
}
