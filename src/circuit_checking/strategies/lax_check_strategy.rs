use crate::circuit_checking::checkers::infinite_loop_checker::InfiniteLoopChecker;
use crate::models::circuit::Circuit;
use crate::circuit_checking::checker_message::CheckerMessage;
use crate::circuit_checking::check_strategy::CheckStrategy;
use crate::circuit_checking::checker::Checker;
use std::process::exit;

pub struct LaxCheckStrategy {}
impl CheckStrategy for LaxCheckStrategy {
    fn check(&self, circuit: &Circuit) {
        let mut infinite_checker = InfiniteLoopChecker::new();
        //let mut unused_checker = UnusedChecker::new();

        //match unused_checker.check(&circuit) {
        //    Some(checker_message) => {
        //        match checker_message {
        //            CheckerMessage::Error(message) => {
        //                println!("{}", message);
        //                exit(1);
        //            },
        //            CheckerMessage::Warning(message) => println!("{}", message)
        //        }
        //    },
        //    None => println!("No errors")
        //}
        match infinite_checker.check(&circuit) {
            Some(checker_message) => {
                match checker_message {
                    CheckerMessage::Error(message) => {
                        println!("{}", message);
                        //exit(1);
                    },
                    CheckerMessage::Warning(message) => println!("{}", message)
                }
            },
            None => println!("No errors")
        }
    }
}