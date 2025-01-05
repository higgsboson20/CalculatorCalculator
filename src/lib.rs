pub mod process;
pub mod gui;
pub mod math;

#[derive(PartialEq)]
pub enum ErrorState {
    NoneState,
    IncorrectEquation,
    ImproperNumberOfValues,
    
}