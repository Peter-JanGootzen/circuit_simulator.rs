use crate::models::circuit::Circuit;

pub trait CheckStrategy {
    fn check(&self, circuit: &Circuit);
}