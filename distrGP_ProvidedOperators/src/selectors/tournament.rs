extern crate rand;
extern crate distrGP_Generator;

use self::distrGP_Generator::Graph;
use self::distrGP_Generator::Selector;

use self::rand::distributions::{IndependentSample, Range};

#[derive(Debug,Clone)]
pub struct Tournament
{
	tournament_size:u32,

}

impl Tournament
{

	pub fn new(size:u32) -> Tournament
	{
		Tournament{tournament_size:size}
	}

}

impl Selector for Tournament
{

	fn get_copy(&self) ->  Box<Selector>
	{

		Box::new(self.clone()) as Box<Selector>
	
	}
	fn select(&self, pop: &Vec<Graph>) -> Vec<Graph>
	{

			let mut rng = rand::thread_rng();
			let graph_count = Range::new(0, pop.len());



			//inefficient for small tournament sizes
			let mut tournament_vector = Vec::new();

			for _ in 0 .. self.tournament_size
			{
				tournament_vector.push(pop[graph_count.ind_sample(&mut rng)].clone());

			}
			tournament_vector.sort();

		
		
			let mut final_graph = Vec::new();
			final_graph.push(tournament_vector[0].clone());
			final_graph




	}

}
