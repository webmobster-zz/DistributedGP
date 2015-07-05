#[derive(Debug,Clone)]
pub struct OperatorMerge
{
	probability:f32,
	depth: u64
}

impl OperatorMerge
{

	pub fn new(probability: f32, depth: u64) -> OperatorMerge
	{
		OperatorMerge{probability: probability, depth: depth}
	}

}

impl GeneticOperator for OperatorMerge
{


	fn get_copy(&self) ->  Box<GeneticOperator>
	{

		Box::new(self.clone()) as Box<GeneticOperator>
	
	}

	fn get_probability(&self) ->  f32
	{

		self.probability
	
	}

	fn operate(&self,  map: &mut OperatorMap,selector_closure: &Box<Fn() -> (Graph,Vec<u64>)>) -> Vec<Graph>
	{
		let mut rng = rand::weak_rng();

		//println!("before tree mutation: {:?}",working_graph); 
		//fast but bad
		let mut rng = rand::weak_rng();

		//fix bad OO practices, getters and setters etc
		let graph_length = Range::new(0, working_graph.get_size());



		let mut unfinished_nodes = VecDeque::new();

		unfinished_nodes.push(graph_length.ind_sample(&mut rng));

		for i in range 0..self.depth
		{

			let mut next1 =None;	let mut next2 =None;


			while !unfinished_nodes.is_empty()
			{
				let working_index = unfinished_nodes.pop()

				let mut working_node = working_graph.get_node(working_index);

				let Node(op, suc1,suc2,) = current_node.clone();
				next1=suc1, next2=suc2;
				


			}
			if next1.is_some();
			{
				unfinished_nodes.push(next1);
			}
			if next2.is_some();
			{
				unfinished_nodes.push(next2);

			}

		}


		
	}
}

#[derive(Debug,Clone)]
pub struct OperatorSplit
{

	probability:f32
}

impl OperatorSplit
{

	pub fn new(probability: f32) -> OperatorSplit
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

	fn operate(&self,  _: &Vec<OperatorReference>,selector_closure: &Box<Fn() -> Graph>) -> Vec<Graph>
	{

	}
}
	
