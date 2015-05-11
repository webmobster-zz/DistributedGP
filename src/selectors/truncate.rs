pub use super::super::graph::Graph;

pub struct Truncate
{
	current_graph: usize
}

impl Truncate;
{

	fn init() -> Truncate
	{
		Truncate{current_graph:0}
	}

}

impl SelectorTrait for Tournament
{
	//inefficient
	pub fn select(&mut self, pop: &Vec<Graph>) -> Graph
	{

		(&mut **pop).sort();

		let graph=pop[self.current_graph];
		self.current_graph+= 1;
		graph


	}
	pub fn reset(&mut self)
	{
		self.current_graph=0;
	}
}







