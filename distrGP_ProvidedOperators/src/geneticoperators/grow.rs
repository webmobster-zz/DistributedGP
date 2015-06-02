extern crate rand;
extern crate distrGP_Generator;

use self::distrGP_Generator::Graph;
use self::distrGP_Generator::Node;
use self::distrGP_Generator::GeneticOperator;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::RandomKey;


use self::rand::distributions::{IndependentSample, Range};


#[derive(Debug,Clone)]
pub struct StandardGrow
{

	probability:f32,
	size: u32
}

impl StandardGrow
{

	pub fn new(probability: f32, size: u32) -> StandardGrow
	{
		StandardGrow{probability: probability, size: size}
	}

}

impl GeneticOperator for StandardGrow
{


	fn get_copy(&self) ->  Box<GeneticOperator>
	{

		Box::new(self.clone()) as Box<GeneticOperator>
	
	}

	fn get_probability(&self) ->  f32
	{

		self.probability
	
	}


	fn operate(&self, map: &mut OperatorMap, _: &Box<Fn() -> Graph>) -> Vec<Graph>
	{

		if map.len() ==0
		{
			panic!("no operators defined");

		}

		let mut end_operators = Vec::new();
		
		for entry in map.iter()
		{
			let (hash,op)=entry;
			if op.get_sucessors() == 0
			{
				end_operators.push(*hash);
			}

		}
		

		if end_operators.len() ==0
		{
			panic!("no end point operators defined");

		}


		let mut new_graph: Graph = Graph::empty_graph();




		//fast but bad
	   	let mut rng = rand::weak_rng();


		//intial node
		let operator = map.random_key(&mut rng);
		let mut loose_ends = Vec::new();

		let end = new_graph.get_size();
		new_graph.add_to_end(Node(operator,None,None));
		loose_ends.push(end);


		//Grow behaviour comes from here
		loop
		{	



			while new_graph.get_size() < self.size as usize && loose_ends.len() != 0
			{


				//get loose end
				let working_index = loose_ends.pop().unwrap();
			

				let node = new_graph.get_node(working_index);

				let Node(op,suc1,suc2) = node;
			
				assert!(suc1.is_none() && suc2.is_none(),"Invalid Node");

				let end = new_graph.get_size();
			
				
				if map[&op].get_sucessors() ==2
				{
					//replace unlinked node with node with links
					new_graph.set_node(working_index,Node(op,Some(end),Some(end +1)));

					//add new node with unfilled sucessors to the ends
					new_graph.add_to_end(Node( map.random_key(&mut rng),None,None));
					new_graph.add_to_end(Node( map.random_key(&mut rng),None,None));
					loose_ends.push(end);
					loose_ends.push(end+1);

				}
				else if  map[&op].get_sucessors() ==1
				{

					new_graph.set_node(working_index,Node(op,Some(end),None));

					//add new node with unfilled sucessors to the ends
					new_graph.add_to_end(Node(map.random_key(&mut rng),None,None));
					loose_ends.push(end);

				}
				else if  map[&op].get_sucessors() ==0
				{
					
					//do nothing, as the Node should have unconnected sucessors already


				}



			}


			//CLEAR UP DANGLING NODES
			

			//no further succesors possible
			if loose_ends.len() == 0
			{
				break;
			}


			let end_operator_count = Range::new(0, end_operators.len());
			
			//get a random index from the end operator list, which is used to get an operator from the operator list
			//SAME OPERATOR EVERY TIME FIX THIS!
			let operator = end_operators[end_operator_count.ind_sample(&mut rng)];

			let working_index = loose_ends.pop().unwrap();
			
			let node = new_graph.get_node(working_index);

			let Node(op,_,_) = node;

			let end = new_graph.get_size();


			if  map[&op].get_sucessors() == 3
			{
				panic!("unimplemented feature");

			}
			
			else if  map[&op].get_sucessors() == 2
			{
				//replace unlinked node with node with links
				new_graph.set_node(working_index,Node(op,Some(end),Some(end +1)));

				//add new node with unfilled sucessors to the ends
				new_graph.add_to_end(Node(operator.clone(),None,None));
				new_graph.add_to_end(Node(operator.clone(),None,None));
				loose_ends.push(end);
				loose_ends.push(end+1);

			}
			else if  map[&op].get_sucessors() == 1
			{
				//replace unlinked node with node with link
				new_graph.set_node(working_index,Node(op,Some(end),None));

				//add new node with unfilled sucessors to the ends
				new_graph.add_to_end(Node(operator.clone(),None,None));
				loose_ends.push(end);

			}
			else if map[&op].get_sucessors() ==0
			{

				//do nothing, as the Node should have unconnected sucessors already
				//hopefully this will get compiled away, usefull for code readability sakes



			}

		}
		vec!(new_graph)

	}
}


