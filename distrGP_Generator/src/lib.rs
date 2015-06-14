//#![deny(warnings)]

extern crate rand;


pub use self::graph::Graph;
pub use self::graph::Node;

pub use self::geneticoperator::GeneticOperator;
pub use self::selectortrait::Selector;

pub use self::operator::OperatorMap;
pub use self::operator::Operator;
pub use self::operator::RandomKey;
pub use self::operator::SpecialOperator;

pub use self::state::GlobalState;
pub use self::state::LocalState;
pub use self::state::StateIO;

pub use self::individualcomm::IndividualComm;

use self::rand::Rng;



mod graph;
mod operator;
mod state;
mod selectortrait;
mod reproduce;
mod individualcomm;


mod geneticoperator;

//#[derive(Show,Clone)]
pub struct Generator
{
	popcount: u32,
	graph_list: Vec<GlobalState>,
	operatorpointers: OperatorMap,
	crossmut: Vec<Box<GeneticOperator>>,
	grow_operator: Box<GeneticOperator>,
	selector: Box<selectortrait::Selector>,
	life: u64,
	population_UUID: [u64; 2]




}



impl Generator
{
	pub fn init(popcount: u32, operators: OperatorMap,selector: Box<Selector>, crossmut:Vec<Box<GeneticOperator>>,grow_operator: Box<GeneticOperator>,life: u64) -> Generator
	{
		
		let graph = Vec::with_capacity(popcount as usize);

		let mut rng = rand::thread_rng();

		let generator = Generator {
				popcount:popcount,
				graph_list:graph,

				operatorpointers : operators, 
				crossmut: crossmut,
				grow_operator: grow_operator,
				selector: selector,

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

	pub fn initialize_graphs( &mut self) -> Vec<IndividualComm>
	{

		let mut comms: Vec<IndividualComm> = Vec::new();
		for i in 0..self.graph_list.len()
		{	
			
			let (tx,rx) = self.graph_list[i].initialize(self.life);
			comms.push(IndividualComm::new(tx,rx));

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
				let mut new_graph = x.clone();
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
		self.graph_list = reproduce::reproduce(&self.selector,self.graph_list.clone(), &self.crossmut, &mut self.operatorpointers);

		
		
	}


	pub fn get_graph_list(&self) -> Vec<GlobalState>
	{
		self.graph_list.clone()
		
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

	




	pub fn get_selector(&self) -> Box<selectortrait::Selector>
	{
		self.selector.clone()
		
	}

}



