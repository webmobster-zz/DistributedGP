#[derive(Debug,Clone)]
pub struct OperatorMerge
{

	probability:f32,
	samples: usize
}

impl OperatorMerge
{

	pub fn new(probability: f32) -> OperatorMerge
	{
		TreeCross{probability: probability}
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

	fn operate(&self,  _: &Vec<OperatorReference>,selector_closure: &Box<Fn() -> Graph>) -> Vec<Graph>
	{
	
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
	
