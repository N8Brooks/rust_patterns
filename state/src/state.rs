use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum GumballMachineStateId {
    NoQuarterState,
    HasQuarterState,
    SoldOutState,
    SoldState,
}

#[derive(Debug, PartialEq)]
pub enum GumballMachineStateError {
    NoQuarterInserted,
    AlreadyHasQuarter,
    CrankHasNotBeenTurned,
    AlreadyTurnedCrank,
    OutOfGumballs,
}

pub trait GumballMachineState: Debug {
    fn get_id(&self) -> GumballMachineStateId;

    fn get_count(&self) -> u32;

    fn insert_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn eject_quarter(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn turn_crank(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;

    fn dispense(&mut self) -> Result<Box<dyn GumballMachineState>, GumballMachineStateError>;
}
