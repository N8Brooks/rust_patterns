use crate::{
    has_quarter_state::HasQuarterState,
    state::{GumballMachineState, GumballMachineStateError, GumballMachineStateId},
};

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

    fn insert_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        let state: &dyn GumballMachineState = self;
        Ok(Box::new(HasQuarterState::from(state)))
    }

    fn eject_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::NoQuarterInserted)
    }

    fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::NoQuarterInserted)
    }

    fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::NoQuarterInserted)
    }
}

impl std::convert::From<&dyn GumballMachineState> for NoQuarterState {
    fn from(state: &dyn GumballMachineState) -> Self {
        NoQuarterState {
            count: state.get_count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_quarter() {
        let mut state: Box<dyn GumballMachineState> = Box::new(NoQuarterState::default());
        state = state.insert_quarter().unwrap();
        assert_eq!(state.get_id(), GumballMachineStateId::HasQuarterState);
        assert_eq!(state.get_count(), 0);
    }

    #[test]
    fn eject_quarter() {
        let mut state = Box::new(NoQuarterState::default());
        assert_eq!(
            state.eject_quarter().unwrap_err(),
            GumballMachineStateError::NoQuarterInserted
        );
    }

    #[test]
    fn turn_crank() {
        let mut state = Box::new(NoQuarterState::default());
        assert_eq!(
            state.turn_crank().unwrap_err(),
            GumballMachineStateError::NoQuarterInserted
        );
    }

    #[test]
    fn dispense() {
        let mut state = Box::new(NoQuarterState::default());
        assert_eq!(
            state.dispense().unwrap_err(),
            GumballMachineStateError::NoQuarterInserted
        );
    }
}
