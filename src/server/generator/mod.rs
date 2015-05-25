
pub use self::graph::Graph;
pub use self::geneticoperator::GeneticOperator;
pub use self::selectortrait::Selector;

use self::operator::Operator;
use self::operator::OperatorTrait;


pub mod graph;
pub mod operator;
mod selectortrait;
mod reproduce;

mod geneticoperator;

//#[derive(Show,Clone)]
pub struct Generator
{
	popcount: u32,
	graph_list: Vec< Graph>,
	initial_tree_size: u32,
	operatorpointers: Vec<Operator>,
	end_operators: Vec<u32>,

	operator_trait: Box<OperatorTrait  + Send>,


	repetitions: u32,

	selector: Box<selectortrait::Selector>,
	life: u32,
	population_UUID: [u64; 2]




}



impl Generator
{
	pub fn init(popcount: u32, initial_tree_size: u32 ,operators: Vec<Operator>, end_operators: Vec<u32>,
	            operator_trait: Box<OperatorTrait + Send>, repetitions: u32, selector: Box<Selector>, life: u32) -> Generator
	{
		
		let graph = Box::new(Vec::with_capacity(popcount as usize));

	

		Generator {
				popcount:popcount,
				graph_list:graph,
				initial_tree_size: initial_tree_size,
				operatorpointers : operators, 
				end_operators: end_operators,
				operator_trait: operator_trait ,

				repetitions: repetitions,
				selector: selector,
				life: life
			  }
	

	}
	pub fn generate_graphs( &mut self)
	{
		for _ in 0 .. self.popcount
		{
			let new_graph = Graph::grow_graph(&self.operatorpointers,&self.end_operators,self.initial_tree_size, self.life);

			self.graph_list.push(new_graph);
		}

	}

	pub fn reproduce(&mut self)
	{


		reproduce::reproduce(self)

		
		
	}



	pub fn get_graph(&self, index: usize) -> Graph
	{
		self.graph_list[index].clone()
		
	}

		
	pub fn set_graphs(&mut self, graphs: Vec< Graph>)
	{

		self.graph_list=graphs;

	}
	#[allow(dead_code)]
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



