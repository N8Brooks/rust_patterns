use crate::{
    no_quarter_state::NoQuarterState,
    sold_out_state::SoldOutState,
    state::{GumballMachineState, GumballMachineStateError, GumballMachineStateId},
};

#[derive(Debug, Default)]
pub struct SoldState {
    count: u32,
}

impl SoldState {
    fn release_ball(&mut self) {
        self.count -= 1;
    }
}

impl GumballMachineState for SoldState {
    fn get_id(&self) -> GumballMachineStateId {
        GumballMachineStateId::SoldState
    }

    fn get_count(&self) -> u32 {
        self.count
    }

    fn insert_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::AlreadyTurnedCrank)
    }

    fn eject_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::AlreadyTurnedCrank)
    }

    fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        Err(GumballMachineStateError::AlreadyTurnedCrank)
    }

    fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError> {
        self.release_ball();
        let state: &dyn GumballMachineState = self;
        if self.count > 0 {
            Ok(Box::new(NoQuarterState::from(state)))
        } else {
            Ok(Box::new(SoldOutState::from(state)))
        }
    }
}

impl std::convert::From<&dyn GumballMachineState> for SoldState {
    fn from(state: &dyn GumballMachineState) -> Self {
        SoldState {
            count: state.get_count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_quarter() {
        let mut state = Box::new(SoldState::default());
        assert_eq!(
            state.insert_quarter().unwrap_err(),
            GumballMachineStateError::AlreadyTurnedCrank,
        );
    }

    #[test]
    fn eject_quarter() {
        let mut state = Box::new(SoldState::default());
        assert_eq!(
            state.eject_quarter().unwrap_err(),
            GumballMachineStateError::AlreadyTurnedCrank,
        );
    }

    #[test]
    fn turn_crank() {
        let mut state = Box::new(SoldState::default());
        assert_eq!(
            state.turn_crank().unwrap_err(),
            GumballMachineStateError::AlreadyTurnedCrank,
        );
    }

    #[test]
    fn dispense_to_not_sold_out() {
        let mut state = Box::new(SoldState { count: 2 });
        assert_eq!(
            state.dispense().unwrap().get_id(),
            GumballMachineStateId::NoQuarterState,
        );
    }

    #[test]
    fn dispense_to_sold_out() {
        let mut state = Box::new(SoldState { count: 1 });
        assert_eq!(
            state.dispense().unwrap().get_id(),
            GumballMachineStateId::SoldOutState,
        );
    }
}
