//#![deny(warnings)]

extern crate rand;


pub use self::graph::Graph;
pub use self::graph::Node;

pub use self::geneticoperator::GeneticOperator;
pub use self::selectortrait::Selector;
pub use self::operator::Operator;
pub use self::operator::OperatorTrait;

use self::rand::Rng;



pub mod graph;
pub mod operator;
mod selectortrait;
mod reproduce;

mod geneticoperator;

//#[derive(Show,Clone)]
pub struct Generator
{
	popcount: u32,
	graph_list: Vec<Graph>,
	operatorpointers: Vec<Operator>,
	operator_trait: Box<OperatorTrait  + Send>,
	crossmut: Vec<Box<GeneticOperator>>,
	grow_operator: Box<GeneticOperator>,

	repetitions: u32,

	selector: Box<selectortrait::Selector>,
	life: u32,
	population_UUID: [u64; 2]




}



impl Generator
{
	pub fn init(popcount: u32, operators: Vec<Operator>,
	            operator_trait: Box<OperatorTrait + Send>, repetitions: u32, selector: Box<Selector>, crossmut:Vec<Box<GeneticOperator>>,grow_operator: Box<GeneticOperator>, life: u32) -> Generator
	{
		
		let graph = Vec::with_capacity(popcount as usize);

		let mut rng = rand::thread_rng();

		let generator = Generator {
				popcount:popcount,
				graph_list:graph,

				operatorpointers : operators, 
				operator_trait: operator_trait ,
				crossmut: crossmut,
				grow_operator: grow_operator,

				selector: selector,

				repetitions: repetitions,
				life: life,
				population_UUID: [rng.gen::<u64>(); 2]
			  };
		assert!(generator.crossmut.len() > 0,"Need to select at least one crossover/mutation genetic operators");
		assert!(generator.check_crossmut(),"Genetic Operator probabilties don't add up to 1.0");

		generator
	}

	fn check_crossmut(&self) -> bool
	{
		let mut running_total = 0.0;

		for i in 0..self.crossmut.len()
		{
			running_total+=self.crossmut[i].get_probability();
		}
		if running_total == 1.0
		{
			true
		}
		else
		{
			false
		}
	}

	//a bit hacky
	pub fn generate_graphs( &mut self)
	{

		let closure = self.selector.select(vec!());
		
		while self.graph_list.len() < self.popcount as usize
		{
			let new_graph = self.grow_operator.operate(&self.operatorpointers,&closure);
			for x in new_graph
			{
				let mut new_graph = x.clone();
				new_graph.set_life(self.life);
				self.graph_list.push(new_graph);
				if self.graph_list.len() == self.popcount as usize
				{
						break;
				}

			}

		}
	}

	pub fn reproduce(&mut self)
	{

		//think about how to deal with this clone
		self.graph_list = reproduce::reproduce(&self.selector,self.graph_list.clone(), &self.crossmut, &self.operatorpointers);

		
		
	}


	pub fn get_graph_list(&self) -> Vec<Graph>
	{
		self.graph_list.clone()
		
	}


	pub fn get_graph(&self, index: usize) -> Graph
	{
		self.graph_list[index].clone()
		
	}

		
	pub fn set_graphs(&mut self, graphs: Vec< Graph>)
	{

		self.graph_list=graphs;

	}

	pub fn get_popcount(&self) -> u32
	{
		self.popcount
		
	}
	pub fn get_operator_trait(&self) -> Box<OperatorTrait + Send>
	{
		self.operator_trait.clone()
		
	}


	pub fn get_repetitions(&self) -> u32
	{
		self.repetitions.clone()
		
	}




	pub fn get_selector(&self) -> Box<selectortrait::Selector>
	{
		self.selector.clone()
		
	}

}



