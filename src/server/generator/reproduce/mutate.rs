extern crate rand;

use super::super::Generator;
use super::super::Graph;
use super::super::graph::Node;
use super::selector::SelectionType::{Tournament};
use super::selector;


use self::rand::distributions::{IndependentSample, Range};


pub fn point_mutate(generator: & mut Generator) -> Graph
{

	let selection_type = generator.get_selection_type();
	
	let mut working_graph= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};

	if generator.get_parents()
	{

		let mut parent1 = working_graph.clone();
		parent1.remove_parents();
		working_graph.add_parents(Some(parent1),None);

	}
	//println!("before tree mutation: {:?}",working_graph); 
	//fast but bad
	let mut rng = rand::weak_rng();

	//fix bad OO practices, getters and setters etc
	let graph_length = Range::new(0, working_graph.list.len());

	let working_index = graph_length.ind_sample(&mut rng);

	let mut working_node = working_graph.list[working_index].clone();

	//assume sucessor count is correct
	let Node(_,mut suc1,mut suc2,mut suc3) = working_node.clone();




	let operator_count = Range::new(0, generator.operatorpointers.len());

	let new_operator = generator.operatorpointers[operator_count.ind_sample(&mut rng)].clone();



	// has to be a nicer way

	//matches successors, either prunes or generates a random sucessor if non exist.
 
	if new_operator.sucessors == 0
	{

		working_node=Node(new_operator.clone(),-1,-1,-1)

	}

	if new_operator.sucessors == 1
	{
		if suc1 == -1
		{
			suc1 = graph_length.ind_sample(&mut rng) as isize;

		}

		working_node=Node(new_operator.clone(),suc1,-1,-1)

	}
	if new_operator.sucessors == 2
	{

		if suc1 == -1
		{
			suc1 = graph_length.ind_sample(&mut rng) as isize;

		}

		if suc2 == -1
		{
			suc2 = graph_length.ind_sample(&mut rng) as isize ;

		}

		working_node=Node(new_operator.clone(),suc1,suc2,-1)

	}

	if new_operator.sucessors == 3
	{


		if suc1 == -1
		{
			suc1 = graph_length.ind_sample(&mut rng) as isize;

		}

		if suc2 == -1
		{
			suc2 = graph_length.ind_sample(&mut rng) as isize;

		}
		if suc3 == -1
		{
			suc3 = graph_length.ind_sample(&mut rng) as isize;

		}



		working_node=Node(new_operator.clone(),suc1,suc2,suc3)

	}


	working_graph.list[working_index]=working_node;


	//println!("point tree mutation: {:?}",working_graph); 
	working_graph

}



pub fn tree_mutate(generator: & mut Generator) -> Graph
{

	let selection_type = generator.get_selection_type();
	
	let mut working_graph= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};

	if generator.get_parents()
	{

		let mut parent1 = working_graph.clone();
		parent1.remove_parents();
		working_graph.add_parents(Some(parent1),None);

	}
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
	//println!("rm");
	let selection_type = generator.get_selection_type();
	
	let mut working_graph= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};

	if generator.get_parents()
	{

		let mut parent1 = working_graph.clone();
		parent1.remove_parents();
		working_graph.add_parents(Some(parent1),None);

	}

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
	//println!("clean");
	let selection_type = generator.get_selection_type();
	
	let mut working_graph= match selection_type
	{
				Tournament(k) => selector::tournament_selection(generator,k),
				_ => panic!("unimplemented code")
	};
	if generator.get_parents()
	{

		let mut parent1 = working_graph.clone();
		parent1.remove_parents();
		working_graph.add_parents(Some(parent1),None);

	}


	working_graph.clean();

	working_graph

}

