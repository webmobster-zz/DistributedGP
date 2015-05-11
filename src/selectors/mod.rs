


#[derive(Debug,Clone)]
pub trait SelectorTrait
{
	fn select(&self, pop: &Vec<Graph>) -> Graph
	fn reset(&self) -> Graph
}



