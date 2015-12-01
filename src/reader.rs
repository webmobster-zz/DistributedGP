


use distrgp_generator::Selector;
use distrgp_generator::OperatorMap;

use std::collections::HashSet;


use rand::distributions::{IndependentSample, Range};
use rand::Rng;




use std::u32;



pub struct ProblemDescription {
    popcount: u32,
    selector: Box<Selector>,
    life: u32,
}



pub fn readfile() -> ProblemDescription {
    ProblemDescription {
        popcount: 50,
        selector: Box::new(::distrgp_providedoperators::selectors::Tournament::new(2)) as Box<Selector>,
        life: 20000,
    }
}

impl ProblemDescription
{

    pub fn get_popcount(&self) -> u32 {
        self.popcount
    }


    pub fn get_selector(&self) -> Box<Selector> {
        self.selector.clone()

    }

    pub fn get_life(&self) -> u32 {
        self.life
    }


}




pub fn num_cpus() -> usize {
    unsafe {
        return rust_get_num_cpus() as usize;
    }

    extern {
        fn rust_get_num_cpus() -> u64;
    }
}
