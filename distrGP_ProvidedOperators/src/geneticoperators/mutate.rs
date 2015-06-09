extern crate rand;
extern crate distrGP_Generator;

use self::distrGP_Generator::Graph;
use self::distrGP_Generator::Node;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::GeneticOperator;
use self::distrGP_Generator::RandomKey;

use self::rand::distributions::{IndependentSample, Range};
use std::collections::VecDeque;
use std::collections::HashMap;


#[derive(Debug,Clone)]
pub struct PointMutate
{

	probability:f32
}

impl PointMutate
{

	pub fn new(probability: f32) -> PointMutate
	{
		PointMutate{probability: probability}
	}

}

impl GeneticOperator for PointMutate
{


	fn get_copy(&self) ->  Box<GeneticOperator>
	{

		Box::new(self.clone()) as Box<GeneticOperator>
	
	}

	fn get_probability(&self) ->  f32
	{

		self.probability
	
	}

	fn operate(&self,  map: &mut OperatorMap,selector_closure: &Box<Fn() -> (Graph,Vec<u64>)>) -> Vec<(Graph,Vec<u64>)>
	{
	
		let (mut working_graph,vec)= selector_closure();


		//println!("before tree mutation: {:?}",working_graph); 
		//fast but bad
		let mut rng = rand::weak_rng();

		//fix bad OO practices, getters and setters etc
		let graph_length = Range::new(0, working_graph.get_size());

		let working_index = graph_length.ind_sample(&mut rng);

		let mut working_node = working_graph.get_node(working_index);

		//assume sucessor count is correct
		let Node(_,mut suc1,mut suc2) = working_node.clone();


		let new_operator = map.random_key(&mut rng);



		// has to be a nicer way

		//matches successors, either prunes or generates a random sucessor if non exist.
	 
		if map[&new_operator].get_sucessors() == 0
		{

			working_node=Node(new_operator.clone(),None,None)

		}

		if map[&new_operator].get_sucessors() == 1
		{
			if suc1 == None
			{
				suc1 = Some(graph_length.ind_sample(&mut rng));

			}

			working_node=Node(new_operator.clone(),suc1,None)

		}
		if map[&new_operator].get_sucessors() == 2
		{

			if suc1 == None
			{
				suc1 = Some(graph_length.ind_sample(&mut rng));

			}

			if suc2 == None
			{
				suc2 = Some(graph_length.ind_sample(&mut rng));

			}

			working_node=Node(new_operator.clone(),suc1,suc2)

		}



		working_graph.set_node(working_index,working_node);


		//println!("point tree mutation: {:?}",working_graph); 
		vec!((working_graph,vec))
	}
}


/*
pub fn tree_mutate(generator: & mut Generator) -> Graph
{

	let mut working_graph= selector.select();
	
	let mut working_graph= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};


	//println!("before tree mutation: {:?}",working_graph); 


	//fast but bad
	let mut rng = rand::weak_rng();


	let graph_length = Range::new(0, working_graph.list.len());

	let working_index = graph_length.ind_sample(&mut rng);


	//hard coded subtree size
	working_graph.grow_new_subtree(&generator.operatorpointers,&generator.end_operators,working_index,5);


	//working_graph.clean();

	//println!("after tree mutation: {:?}",working_graph); 
	working_graph

}


pub fn point_remove(generator: & mut Generator) -> Graph
{

	let mut working_graph= selector.select();



	//println!("before tree mutation: {:?}",working_graph); 
	//fast but bad
	let mut rng = rand::weak_rng();

	//fix bad OO practices, getters and setters etc
	let graph_length = Range::new(0, working_graph.list.len());

	let working_index = graph_length.ind_sample(&mut rng);

	if working_graph.list.len() > 1
	{
		working_graph.remove_node(working_index);
	}

	working_graph

}


pub fn clean(generator: & mut Generator) -> Graph
{
	let mut working_graph= selector.select();


	working_graph.clean();

	working_graph

}
*/

