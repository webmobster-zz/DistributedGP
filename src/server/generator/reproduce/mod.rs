extern crate rand;

use super::Generator;
use super::graph::Graph;




use self::rand::Rng;


pub mod selector;
mod mutate;
mod crossover;

pub fn reproduce(generator: & mut Generator)
{


	let mut rng = rand::weak_rng();

	let mut weights: Vec<f32> = Vec::new();

	let mut runningtotal = generator.point_mutate_probability;
	weights.push(runningtotal);


	runningtotal = runningtotal + generator.tree_mutate_probability;
	weights.push(runningtotal);

	runningtotal = runningtotal + generator.crossover_probability;
	weights.push(runningtotal);

	runningtotal = runningtotal + generator.flat_crossover_probability;
	weights.push(runningtotal);

	runningtotal = runningtotal + generator.point_remove_probability;
	weights.push(runningtotal);

	runningtotal = runningtotal + generator.clean_probability;
	weights.push(runningtotal);

	if runningtotal > 1.0 {panic!("probabilties larger than 1")}
	let mut newpop: Vec<Graph> = Vec::new();

	

	for _ in 0 .. generator.popcount
	{

		newpop.push(match rng.gen::<f32>()
		{	

			//looks fine so maybe working
			sample if sample < weights[0] => mutate::point_mutate(generator),
			//broken
			sample if sample < weights[1] => mutate::tree_mutate(generator),
			//broken
			sample if sample < weights[2] => crossover::tree_crossover(generator),
			//looks fine so very maybe working
			sample if sample < weights[3] => crossover::flat_crossover(generator),
			//looks fine probably working
			sample if sample < weights[4] => mutate::point_remove(generator),
			//broke
			sample if sample <= weights[5] => mutate::clean(generator),
			_ => {panic!("probabilties didnt add up to 1");}
		});
		
		

	}
	/*match rng.gen::<f32>()
		{
			sample if sample < weights[0] => println!("point: {:?}", sample),
			sample if sample < weights[1] => println!("tree mut: {:?}", sample),
			sample if sample < weights[2] => println!("tree cross: {:?}", sample),
			sample if sample < weights[3] => println!("flat cross: {:?}", sample),
			sample if sample < weights[4] => println!("point rm: {:?}", sample),
			sample if sample < weights[5] => println!("clean: {:?}", sample),
			sample => {panic!("probabilties didnt add up to 1");}
		};
	
	*/
	generator.graph_list=Box::new(newpop);
	//intial node

	//println!("{} new graphs", generator.popcount -generator.graph_list.len());
	


	if generator.graph_list.len() != generator.popcount as usize
	{
		panic!("wrong number of graphs")
	}
}

/*pub fn new_graphs()
{




}

*/
