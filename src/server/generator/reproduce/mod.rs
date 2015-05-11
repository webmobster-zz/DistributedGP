extern crate rand;

use super::Generator;
use super::graph::Graph;
use self::rand::Rng;



//finish this
pub fn reproduce(selector: &mut Box<SelectorTrait>, pop: &Vec<Graph>, crossmut: ?)
{


	let mut rng = rand::weak_rng();

	let mut weights: Vec<f32> = Vec::new();

	let mut newpop: Vec<Graph> = Vec::new();

	for _ in 0 .. generator.popcount
	{
		let sample = rng.gen::<f32>()

		//unchecked for correctness
		let mut runningtotal = 0.0;
		for i in 0...crossmut.size()
		{
			running_total+=crossmut.get(i).get_probability;


			if sample < newpop[i]
			{
				newpop.push(crossmut.get(i).operatate(selector));
				break;
			}
		}
	}

	generator.graph_list=newpop;


	if generator.graph_list.len() != generator.popcount as usize
	{
		panic!("wrong number of graphs")
	}
}


