pub trait IFields {
    type MoveValue: Default + Clone;
    type Value: Default + Clone + Copy;

    fn set_value(&mut self, m: Self::MoveValue, value: Self::Value) -> bool;

    fn has_winner(&mut self) -> bool;

    fn reset(&mut self);
}