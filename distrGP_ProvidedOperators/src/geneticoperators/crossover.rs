extern crate rand;
extern crate distrGP_Generator;

use self::distrGP_Generator::Graph;
use self::distrGP_Generator::Node;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::GeneticOperator;


use self::rand::distributions::{IndependentSample, Range};
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct TreeCross
{

	probability:f32
}

impl TreeCross
{

	pub fn new(probability: f32) -> TreeCross
	{
		TreeCross{probability: probability}
	}

}

impl GeneticOperator for TreeCross
{


	fn get_copy(&self) ->  Box<GeneticOperator>
	{

		Box::new(self.clone()) as Box<GeneticOperator>
	
	}

	fn get_probability(&self) ->  f32
	{

		self.probability
	
	}

	fn operate(&self,  map: &mut OperatorMap,selector_closure: &Box<Fn() -> Graph>) -> Vec<Graph>
	{
		let mut working_graph_parent_one= selector_closure();
		let working_graph_parent_two= selector_closure();

	

		//fast but bad
		let mut rng = rand::weak_rng();

		let graph_length_one = Range::new(0, working_graph_parent_one.get_size());
		let graph_length_two = Range::new(0, working_graph_parent_two.get_size());
		let working_index_one = graph_length_one.ind_sample(&mut rng);
		let working_index_two = graph_length_two.ind_sample(&mut rng);

	
		let mut unfinished_nodes = VecDeque::new();

		let mut index_map: HashMap<usize,usize> = HashMap::new();



		//possibly more efficient way to do this
		index_map.insert(working_index_two,working_index_one);

		unfinished_nodes.push_back((working_index_two,working_index_one));
		




		//increment by one cause position 0 is needs to be allocated to the first node
		let mut last_used_position = working_graph_parent_one.get_size();

		while !unfinished_nodes.is_empty()
		{



			let (current_get_index,current_put_index)= match unfinished_nodes.pop_front()
			{
				Some(x) => x,
				None => panic!("should never happen"),

			};



			//does the current node exist, or is it getting put on the end
			let increase_graph_size;
			

			//should only occur in loops
			if current_put_index< working_graph_parent_one.get_size()
			{
				increase_graph_size=false;
			}
			else if(working_graph_parent_one.get_size()) ==current_put_index
			{

				increase_graph_size=true;
			}
			else
			{
				panic!("error in logic somewhere");
			}





			//make sure wrapping is accounted for
			let mut current_node: Node =working_graph_parent_two.get_node(current_get_index % working_graph_parent_two.get_size());
			



			let Node(op, suc1,suc2) = current_node.clone();


		

			//THE END LOGIC should be fixed
			if map[&op].get_sucessors() ==2
			{
					assert!(suc1.is_some() && suc2.is_some(),"Invalid Node");

				
					let suc1 = suc1.unwrap();
					let suc2 = suc2.unwrap();

					//problem seems to be here suc1s sucessorts will pop before suc2 does
					if !index_map.contains_key(&suc1) && !index_map.contains_key(&suc2)
					{

						current_node = Node(op,Some(last_used_position),Some((last_used_position +1)));
						
						
						
						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;


						index_map.insert(suc2,last_used_position);
						unfinished_nodes.push_back((suc2,last_used_position));
						last_used_position = last_used_position +1;

					}
					else if index_map.contains_key(&suc1) && index_map.contains_key(&suc2)
					{
						current_node = Node(op,Some(*index_map.get(&suc1).unwrap()),Some(*index_map.get(&suc2).unwrap()));
					}
					else if index_map.contains_key(&suc1)
					{
						current_node = Node(op,Some(*index_map.get(&suc1).unwrap()),Some(last_used_position));
						index_map.insert(suc2,last_used_position);
						unfinished_nodes.push_back((suc2,last_used_position));
						last_used_position = last_used_position +1;
					}
					else
					{
						current_node = Node(op,Some(last_used_position),Some(*index_map.get(&suc2).unwrap()));
						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;

					}
				


			}
			else if map[&op].get_sucessors() ==1
			{
					assert!(suc1.is_some() && suc2.is_none(),"Invalid Node");

					let suc1 = suc1.unwrap();
					if !index_map.contains_key(&suc1)
					{
						current_node = Node(op,Some(last_used_position),None);
						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;
					}
					else
					{
						current_node = Node(op,Some(*index_map.get(&suc1).unwrap()),None);
					}
				


			}
			else if map[&op].get_sucessors() ==0
			{
				assert!(suc1.is_none() && suc2.is_none(),"Invalid Node");




			}


			//replace or grow the list
			if !increase_graph_size
			{
				working_graph_parent_one.set_node(current_put_index,current_node);
			}
			else
			{

				working_graph_parent_one.add_to_end(current_node);

			}


		}

		let working_graph=working_graph_parent_one;

		vec!(working_graph)
		

	}

}



#[derive(Debug,Clone)]
pub struct FlatCross
{

	probability:f32
}

impl FlatCross
{

	pub fn new(probability: f32) -> TreeCross
	{
		TreeCross{probability: probability}
	}

}

impl GeneticOperator for FlatCross
{


	fn get_copy(&self) ->  Box<GeneticOperator>
	{

		Box::new(self.clone()) as Box<GeneticOperator>
	
	}

	fn get_probability(&self) ->  f32
	{

		self.probability
	
	}

	fn operate(&self,  _: &mut OperatorMap,selector_closure: &Box<Fn() -> Graph>) -> Vec<Graph>
	{

		let mut working_graph_parent_one= selector_closure();
		let working_graph_parent_two= selector_closure();

	

		//fast but bad
		let mut rng = rand::weak_rng();
	

		//fix bad OO practices, getters and setters etc
		let graph_length_one = Range::new(0, working_graph_parent_one.get_size());
		let graph_length_two = Range::new(0, working_graph_parent_two.get_size());

		let working_index_one = graph_length_one.ind_sample(&mut rng);

		let working_index_three = graph_length_two.ind_sample(&mut rng);
		let working_index_four = graph_length_two.ind_sample(&mut rng);
	


		working_graph_parent_one.replace_slice(working_index_one,working_graph_parent_two.get_slice(working_index_three,working_index_four));



		vec!(working_graph_parent_one)

	}
}


