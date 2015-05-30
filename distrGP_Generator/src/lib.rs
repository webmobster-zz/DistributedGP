




extern crate rand;


pub use self::graph::Graph;
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
	end_operators: Vec<u32>,

	operator_trait: Box<OperatorTrait  + Send>,
	crossmut: Vec<Box<GeneticOperator>>,

	repetitions: u32,

	selector: Box<selectortrait::Selector>,
	life: u32,
	population_UUID: [u64; 2]




}



impl Generator
{
	pub fn init(popcount: u32, operators: Vec<Operator>, end_operators: Vec<u32>,
	            operator_trait: Box<OperatorTrait + Send>, repetitions: u32, selector: Box<Selector>, crossmut:Vec<Box<GeneticOperator>>,life: u32) -> Generator
	{
		
		let graph = Vec::with_capacity(popcount as usize);

		let mut rng = rand::thread_rng();

		Generator {
				popcount:popcount,
				graph_list:graph,

				operatorpointers : operators, 
				end_operators: end_operators,
				operator_trait: operator_trait ,
				crossmut: crossmut,
				repetitions: repetitions,
				selector: selector,
				life: life,
				population_UUID: [rng.gen::<u64>(); 2]
			  }
	

	}
	pub fn generate_graphs( &mut self)
	{
		//for _ in 0 .. self.popcount
		//{
		//	let new_graph = Graph::grow_graph(&self.operatorpointers,&self.end_operators,self.initial_tree_size, self.life);

		//	self.graph_list.push(new_graph);
		//}
		println!("Stub Method");

	}

	pub fn reproduce(&mut self)
	{


		reproduce::reproduce(&self.selector,&mut self.graph_list, &self.crossmut)

		
		
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



