use crate::models::circuit::Circuit;
use super::checker_message::CheckerMessage;

pub trait Checker<'a> {
    fn check(&mut self, circuit: &'a Circuit) -> Option<CheckerMessage>;
}