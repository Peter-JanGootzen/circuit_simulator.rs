use crate::circuit_checking::checkers::infinite_loop_checker::InfiniteLoopChecker;
use crate::circuit_checking::checker::Checker;
use crate::models::circuit::Circuit;
use crate::models::node::Node;
use crate::models::output::Output;
use std::rc::Rc;
use crate::models::gates::NotGate;
use crate::models::gates::SignalGate;
use std::cell::RefCell;

#[test]
pub fn infiniteloopchecker_test() {
    let false_signal_node = Rc::new(Node::Signal(SignalGate {
        signal: Output::False
    }));

    let not_node = Rc::new(Node::Not(NotGate {
        inputs: RefCell::new(vec![false_signal_node.clone()])
    }));
    let all_nodes: Vec<Rc<Node>> = vec![
        false_signal_node.clone(),
        not_node.clone()
    ];
    let circuit = Circuit::new(RefCell::new(all_nodes), RefCell::new(vec![not_node.clone()]));

    let mut checker = InfiniteLoopChecker::new();
    assert!(checker.check(&circuit).is_none());
}