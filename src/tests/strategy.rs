use crate::circuit_checking::checkers::infinite_loop_checker::InfiniteLoopChecker;
use crate::circuit_checking::checker::Checker;
use crate::models::node::NodeStruct;
use crate::models::node::NodeTrait;
use crate::models::gates::SignalGate;
use crate::models::gates::NotGate;
use crate::models::gates::AndGate;
use crate::models::gates::OrGate;
use crate::models::output::Output;
use crate::models::circuit::Circuit;

//#[test]
//fn infiniteloopchecker_test() {
//    let nodes: Vec<Box<dyn NodeTrait>> = Vec::new();
//
//    let signal_false_node = NodeStruct {
//        inputs: Vec::new(),
//        gate: SignalGate { signal: Output::False }
//    };
//
//    let mut not_node = NodeStruct {
//        inputs: Vec::new(),
//        gate: NotGate
//    };
//
//    let signal_true_node = NodeStruct {
//        inputs: Vec::new(),
//        gate: SignalGate { signal: Output::True }
//    };
//
//    let mut and_node = NodeStruct {
//        inputs: Vec::new(),
//        gate: AndGate
//    };
//    let and_node_box: Box<dyn NodeTrait> = Box::new(and_node);
//    let mut circuit = Circuit {
//        nodes: nodes,
//        last_node: &and_node_box
//    };
//    circuit.nodes.push(and_node_box);
//    circuit.nodes.push(Box::new(signal_false_node));
//    circuit.nodes.push(Box::new(signal_true_node));
//    circuit.nodes.push(Box::new(not_node));
//
//    not_node.inputs.push(&circuit.nodes[1]);
//    and_node.inputs.push(&circuit.nodes[2]);
//    and_node.inputs.push(&circuit.nodes[3]);
//    let mut checker = InfiniteLoopChecker::new();
//    assert!(checker.check(&circuit).is_none());
//}

#[test]
fn infiniteloopchecker_testv2() {
    let signal_false_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
        inputs: Vec::new(),
        gate: SignalGate { signal: Output::False }
    });

    let not_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
        inputs: Vec::new(),
        gate: NotGate
    });

    let signal_true_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
        inputs: Vec::new(),
        gate: SignalGate { signal: Output::True }
    });

    let and_node: Box<dyn NodeTrait> = Box::new(NodeStruct {
        inputs: Vec::new(),
        gate: AndGate
    });
    let circuit = Circuit {
        nodes: vec![&signal_false_node, &not_node, &signal_true_node, &and_node],
        last_node: &and_node
    };
    let mut checker = InfiniteLoopChecker::new();
    assert!(checker.check(&circuit).is_none());
}