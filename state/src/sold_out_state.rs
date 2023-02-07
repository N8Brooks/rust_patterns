use crate::state::{GumballMachineState, GumballMachineStateError, GumballMachineStateId};

#[derive(Debug, Default)]
pub struct SoldOutState;

impl GumballMachineState for SoldOutState {
    fn get_id(&self) -> GumballMachineStateId {
        GumballMachineStateId::SoldOutState
    }

    fn get_count(&self) -> u32 {
        0
    }

    fn insert_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::OutOfGumballs)
    }

    fn eject_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::OutOfGumballs)
    }

    fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::OutOfGumballs)
    }

    fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::OutOfGumballs)
    }
}

impl std::convert::From<&dyn GumballMachineState> for SoldOutState {
    fn from(state: &dyn GumballMachineState) -> Self {
        if state.get_count() != 0 {
            panic!("state is not sold out")
        }
        SoldOutState
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_quarter() {
        let mut state = Box::new(SoldOutState::default());
        assert_eq!(
            state.insert_quarter().unwrap_err(),
            GumballMachineStateError::OutOfGumballs,
        );
    }

    #[test]
    fn eject_quarter() {
        let mut state = Box::new(SoldOutState::default());
        assert_eq!(
            state.eject_quarter().unwrap_err(),
            GumballMachineStateError::OutOfGumballs,
        );
    }

    #[test]
    fn turn_crank() {
        let mut state = Box::new(SoldOutState::default());
        assert_eq!(
            state.turn_crank().unwrap_err(),
            GumballMachineStateError::OutOfGumballs,
        );
    }

    #[test]
    fn dispense() {
        let mut state = Box::new(SoldOutState::default());
        assert_eq!(
            state.dispense().unwrap_err(),
            GumballMachineStateError::OutOfGumballs,
        );
    }
}
