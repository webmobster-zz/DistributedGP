extern crate rand;

use super::Graph;
use super::GeneticOperator;
use super::OperatorMap;
use super::Selector;
use super::GlobalState;

use self::rand::Rng;



//finish this
pub fn reproduce(selector: &Box<Selector>, pop: Vec<GlobalState>, crossmut: &Vec<Box<GeneticOperator>>, operators: &mut OperatorMap) -> Vec<GlobalState>
{


	let mut rng = rand::weak_rng();

	let mut newpop: Vec<GlobalState>= Vec::new();

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
					let (memory,graph) = x.clone();
					newpop.push(GlobalState::new(graph,memory));
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


