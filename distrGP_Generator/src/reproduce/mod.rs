extern crate rand;

use super::Graph;
use super::GeneticOperator;
use super::Operator;
use super::Selector;

use self::rand::Rng;



//finish this
pub fn reproduce(selector: &Box<Selector>, pop: Vec<Graph>, crossmut: &Vec<Box<GeneticOperator>>, operators: &Vec<Operator>) -> Vec<Graph>
{


	let mut rng = rand::weak_rng();

	let mut newpop: Vec<Graph> = Vec::new();

	assert!(pop.len() != 0, "Can't have a population size of 0");

	let length= pop.len();

	let closure = selector.select(pop);

	while newpop.len() < length
	{
		let sample = rng.gen::<f32>();

		//unchecked for correctness
		let mut running_total = 0.0;

		for i in 0..crossmut.len()
		{
			running_total+=crossmut[i].get_probability();


			if sample < running_total
			{
				//FIX
				let new_vec = &*crossmut[i].operate(operators,&closure);
				for x in new_vec
				{
					newpop.push(x.clone());

					if newpop.len() == length
					{
						break;
					}
				}
				
				break;
			}
		}
	}

	


	if length != newpop.len()
	{
		panic!("wrong number of graphs")
	}

	newpop
}


