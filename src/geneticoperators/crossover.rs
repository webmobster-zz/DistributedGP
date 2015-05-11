extern crate rand;

use super::super::Generator;
use super::super::Graph;
use super::selector::SelectionType::{Tournament};
use super::selector;


use self::rand::distributions::{IndependentSample, Range};


pub fn tree_crossover(generator: & mut Generator) -> Graph
{

	let selection_type = generator.get_selection_type();
	
	let mut working_graph_parent_one= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};


	let working_graph_parent_two= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};



	//println!("before tree crossover: {0:?}, {1:?}",working_graph_parent_one,working_graph_parent_two); 
	

	//fast but bad
	let mut rng = rand::weak_rng();

	//fix bad OO practices, getters and setters etc
	let graph_length_one = Range::new(0, working_graph_parent_one.list.len());
	let graph_length_two = Range::new(0, working_graph_parent_two.list.len());

	let working_index_one = graph_length_one.ind_sample(&mut rng);

	let working_index_two = graph_length_two.ind_sample(&mut rng);

	


	working_graph_parent_one.replace_subtree(working_index_one,working_index_two,&working_graph_parent_two);



	let working_graph=working_graph_parent_one;

	



	//println!("point tree mutation: {:?}",working_graph); 
	working_graph

}
pub fn flat_crossover(generator: & mut Generator) -> Graph
{

	let selection_type = generator.get_selection_type();
	
	let mut working_graph_parent_one= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};


	let working_graph_parent_two= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};


	//println!("before tree crossover: {0:?}, {1:?}",working_graph_parent_one,working_graph_parent_two); 
	

	//fast but bad
	let mut rng = rand::weak_rng();

	//fix bad OO practices, getters and setters etc
	let graph_length_one = Range::new(0, working_graph_parent_one.list.len());
	let graph_length_two = Range::new(0, working_graph_parent_two.list.len());

	let working_index_one = graph_length_one.ind_sample(&mut rng);

	let working_index_three = graph_length_two.ind_sample(&mut rng);
	let working_index_four = graph_length_two.ind_sample(&mut rng);
	


	working_graph_parent_one.replace_slice(working_index_one,working_graph_parent_two.get_slice(working_index_three,working_index_four));




	let working_graph=working_graph_parent_one;
	



	//println!("point tree mutation: {:?}",working_graph); 
	working_graph

}
