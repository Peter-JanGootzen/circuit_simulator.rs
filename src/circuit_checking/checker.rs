use crate::models::circuit::Circuit;
use super::checker_message::CheckerMessage;

pub trait Checker {
    fn check(&mut self, circuit: &Circuit) -> Option<CheckerMessage>;
}