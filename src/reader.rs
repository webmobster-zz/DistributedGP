extern crate rand;

extern crate distrGP_Generator;
extern crate distrGP_ProvidedOperators;


use self::distrGP_Generator::Selector;
use self::distrGP_Generator::OperatorMap;

use std::collections::HashSet;


use self::rand::distributions::{IndependentSample, Range};
use self::rand::Rng;




use std::u32;



pub struct ProblemDescription
{
	popcount: u32,
	selector: Box<Selector>,
	operator_map: OperatorMap,
	life: u32
}



pub fn readfile() -> ProblemDescription
{
	let mut map = OperatorMap::new();
	distrGP_ProvidedOperators::operators::load_operators(&mut map);
	ProblemDescription{ 
 		popcount: 50,
		selector: Box::new(distrGP_ProvidedOperators::selectors::Tournament::new(2)) as Box<Selector>,
		life: 20000,
		operator_map: map
		}
}

impl ProblemDescription
{

	pub fn get_popcount(&self)-> u32
	{
		self.popcount
	}
	pub fn get_operators(&self)-> OperatorMap
	{
		self.operator_map.clone()

	}

	pub fn get_selector(&self) -> Box<Selector>
	{
		self.selector.clone()

	}

	pub fn get_life(&self) -> u32
	{
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


