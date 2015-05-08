
pub use self::graph::Graph;
use self::operator::Operator;
use self::operator::OperatorTrait;
pub use self::reproduce::selector::SelectionType;




pub mod graph;
pub mod operator;
mod reproduce;
mod converter;

//#[derive(Show,Clone)]
pub struct Generator
{
	popcount: u32,
	pub graph_list: Box<Vec< Graph>>,
	initial_tree_size: u32,
	operatorpointers: Vec<Operator>,
	end_operators: Vec<u32>,

	operator_trait: Box<OperatorTrait  + Send>,

	parents:bool,
	stats: bool,
	//mutation probabilaties between 0 and 1
	point_mutate_probability: f32,
	tree_mutate_probability: f32,
	crossover_probability: f32,
	flat_crossover_probability: f32,
	point_remove_probability: f32,
	clean_probability: f32,

	repetitions: u32,

	selection_type: SelectionType,
	life: u32




}



impl Generator
{
	pub fn init(popcount: u32, initial_tree_size: u32 ,operators: Vec<Operator>, end_operators: Vec<u32>,
	            operator_trait: Box<OperatorTrait + Send>, parents: bool, stats: bool,
		    point_mutate_probability: f32, tree_mutate_probability: f32, crossover_probability: f32,flat_crossover_probability:f32, point_remove_probability: f32, clean_probability: f32,
		    repetitions: u32, selection_type: SelectionType, life: u32) -> Generator
	{
		
		let graph = Box::new(Vec::with_capacity(popcount as usize));

	

		Generator {
				popcount:popcount,
				graph_list:graph,
				initial_tree_size: initial_tree_size,
				operatorpointers : operators, 
				end_operators: end_operators,
				operator_trait: operator_trait ,
				parents: parents,				
				stats:stats,
				point_mutate_probability: point_mutate_probability,
				tree_mutate_probability: tree_mutate_probability,
				crossover_probability: crossover_probability,
				flat_crossover_probability: flat_crossover_probability,
				point_remove_probability: point_remove_probability,
				clean_probability: clean_probability,

				repetitions: repetitions,
				selection_type: selection_type,
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
	/*

	pub fn get_converted(&self) -> Option<Vec<ConvertedGraph>>
	{
		
	}*/

	#[allow(dead_code)]
	pub fn get_graph(&self, index: usize) -> Graph
	{
		self.graph_list[index].clone()
		
	}

		
	pub fn set_graphs(&mut self, graphs: Box<Vec< Graph>>)
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

	pub fn save_stats(&self) -> bool
	{

		self.stats
	}


	pub fn get_repetitions(&self) -> u32
	{
		self.repetitions.clone()
		
	}



	pub fn get_parents(&self) -> bool
	{
		self.parents
		
	}

	pub fn get_selection_type(&self) -> SelectionType
	{
		self.selection_type.clone()
		
	}

}



