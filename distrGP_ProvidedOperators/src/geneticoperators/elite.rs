extern crate distrGP_Generator;

use self::distrGP_Generator::Graph;
use self::distrGP_Generator::Selector;


#[derive(Debug,Clone)]
pub struct Truncate;

impl Truncate
{

	fn new() -> Truncate
	{
		Truncate
	}

}

impl Selector for Truncate
{


	fn get_copy(&self) ->  Box<Selector>
	{

		Box::new(self.clone()) as Box<Selector>
	
	}
	//inefficient
	fn select(&self, pop: &Vec<Graph>) -> Vec<Graph>
	{
		panic!("broken");
		let mut top =pop.clone();
		top.sort();
		top
		

	}

}







