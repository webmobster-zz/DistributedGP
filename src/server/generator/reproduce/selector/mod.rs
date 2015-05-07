extern crate rand;

pub use super::super::graph::Graph;
pub use super::super::Generator;
use self::SelectionType::{Truncation,Tournament,Roulette};


use self::rand::distributions::{IndependentSample, Range};



#[derive(Debug,Clone)]
pub enum SelectionType
{
	Truncation,
	Tournament(u32),
	Roulette


}

pub fn tournament_selection(generator: & mut Generator, tournament_size: u32) -> Graph
{

		let graph_count = Range::new(0, generator.graph_list.len());

		//fast but bad
	   	let mut rng = rand::weak_rng();

		//inefficient for small tournament sizes
		let mut tournament_vector = Vec::new();

		for _ in 0 .. tournament_size
		{

			tournament_vector.push(generator.graph_list[graph_count.ind_sample(&mut rng)].clone());

		}
		tournament_vector.sort();

		
		
		tournament_vector[0].clone()




}
