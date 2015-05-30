extern crate rand;

use super::Generator;
use super::Graph;
use super::GeneticOperator;
use super::selectortrait::Selector;

use self::rand::Rng;



//finish this
pub fn reproduce(selector: &Box<Selector>, pop: &mut Vec<Graph>, crossmut: &Vec<Box<GeneticOperator>>)
{


	let mut rng = rand::weak_rng();

	let mut weights: Vec<f32> = Vec::new();

	let mut newpop: Vec<Graph> = Vec::new();

	for _ in 0 .. pop.len()
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
				let new_vec = &*crossmut[i].operate(pop,selector);
				for x in new_vec
				{
					newpop.push(x.clone());

				}
				
				break;
			}
		}
	}

	


	if pop.len() != newpop.len()
	{
		panic!("wrong number of graphs")
	}

	*pop=newpop;
}


