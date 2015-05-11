extern crate rand;

pub use super::super::graph::Graph;

use self::rand::distributions::{IndependentSample, Range};

pub struct Tournament
{
	tournament_size:u32,
	rng: Box<Rng>

}

impl Tournament
{

	fn init(size:u32) -> Tournament
	{
		Tournament{tournament_size:size,rng: Box::new(rand::weak_rng())}
	}

}

impl SelectorTrait for Tournament
{

	pub fn select(&mut self, pop: &Vec<Graph>) -> Graph
	{


			let graph_count = Range::new(0, pop.len());



			//inefficient for small tournament sizes
			let mut tournament_vector = Vec::new();

			for _ in 0 .. self.tournament
			{
				tournament_vector.push(pop[graph_count.ind_sample(&mut *self.rng)].clone());

			}
			tournament_vector.sort();

		
		
			tournament_vector[0]




	}
	pub fn reset(&mut self)
	{

	}
}
