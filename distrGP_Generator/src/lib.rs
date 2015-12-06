#![deny(warnings)]
#![feature(custom_derive, plugin,core)]
#![plugin(serde_macros)]
extern crate serde;
extern crate serde_json;

extern crate rand;

pub use self::graph::Graph;
pub use self::graph::Node;

pub use self::geneticoperator::GeneticOperator;
pub use self::selectortrait::Selector;

pub use self::operator::OperatorMap;
pub use self::operator::MinifiedOperator;
pub use self::operator::RandomKey;
pub use self::operator::SpecialOperator;
pub use self::state::GlobalState;
pub use self::state::LocalState;
pub use self::state::StateIO;
pub use self::operator::UUID;
pub use self::bichannel::BiChannel;
pub use operator::operator_compiler::Compiler;

use self::rand::Rng;
use operator::operator_compiler::CompiledOperator;




mod graph;
mod operator;
mod state;
mod selectortrait;
mod reproduce;
mod bichannel;


mod geneticoperator;

//FIXME
#[allow(dead_code)]
pub struct Generator<'a>
{
	popcount: u32,
	graph_list: Vec<GlobalState>,
	operatorpointers: OperatorMap,
	crossmut: Vec<&'a GeneticOperator>,
	compiler: &'a Compiler,
	grow_operator: &'a GeneticOperator,
	selector: &'a selectortrait::Selector,
	life: u64,
	population_uuid: UUID,
	operators: Vec<CompiledOperator>
}



impl<'a> Generator<'a>
{
	pub fn init(popcount: u32, operator_description_path: String, selector: &'a Selector,
		 crossmut:Vec<&'a GeneticOperator>,grow_operator: &'a GeneticOperator,life: u64, compiler: &'a Compiler) -> Generator<'a>{

		let graph = Vec::with_capacity(popcount as usize);

		let mut rng = rand::thread_rng();
		let (compiled, minified) = operator::operator_compiler::load_base_operators(operator_description_path, compiler);
		let mut operator_map = OperatorMap::new();
		for (uuid,mini) in minified
		{
			operator_map.insert(uuid,mini);
		}
		let generator = Generator {
				popcount:popcount,
				graph_list:graph,
				operatorpointers: operator_map,
				crossmut: crossmut,
				grow_operator: grow_operator,
				selector: selector,
				operators: compiled,
				compiler: compiler,
				life: life,
				population_uuid: UUID{x: [rng.gen::<u64>(); 2]}
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



	pub fn initialize_graphs( &mut self) -> Vec<BiChannel<StateIO>>
	{

		let mut comms: Vec<BiChannel<StateIO>> = Vec::new();
		for i in 0..self.graph_list.len()
		{

			let  comm=self.graph_list[i].initialize(self.life);
			comms.push(comm);

		}
		comms


	}

	//a bit hacky
	pub fn generate_graphs( &mut self)
	{

		let closure = self.selector.select(vec!());

		while self.graph_list.len() < self.popcount as usize
		{
			let new_graph = self.grow_operator.operate(&mut self.operatorpointers,&closure);
			for x in new_graph
			{
				let new_graph = x.clone();
				let (graph,memory) = new_graph;
				self.graph_list.push(GlobalState::new(memory,graph));
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
		self.graph_list = reproduce::reproduce(self.selector,self.graph_list.clone(), &self.crossmut, &mut self.operatorpointers);

	}


	pub fn get_graph_list(&self) -> Vec<GlobalState>
	{
		self.graph_list.clone()
	}

	//may not be efficient
	pub fn get_graph_list_safecopy(&self) -> Vec<GlobalState>
	{
		self.graph_list.iter().map(|x| x.clone().unique_copy()).collect()

	}




	pub fn get_graph(&self, index: usize) -> GlobalState
	{
		self.graph_list[index].clone()

	}


	pub fn set_graphs(&mut self, graphs: Vec< GlobalState>)
	{

		self.graph_list=graphs;

	}

	pub fn get_popcount(&self) -> u32
	{
		self.popcount

	}

	pub fn get_operator_map(&self) -> OperatorMap
	{
		self.operatorpointers.clone()

	}

	pub fn get_operator_map_ref(&self) -> &OperatorMap
	{
		& self.operatorpointers

	}
	pub fn get_graph_list_mutref(&mut self) -> &mut Vec<GlobalState>
	{
		&mut self.graph_list

	}






	pub fn get_selector(&self) -> &selectortrait::Selector
	{
		self.selector

	}

}
