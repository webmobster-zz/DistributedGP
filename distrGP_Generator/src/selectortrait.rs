use super::Graph;
use super::GlobalState;

pub trait Selector
{
	fn select(&self, pop: Vec<GlobalState> ) -> Box<Fn() -> (Graph,Vec<u64>)>;
	fn get_copy(&self) -> Box<Selector>;
}


impl Clone for Box<Selector>
{
    fn clone(&self) -> Box<Selector>
    {
        self.get_copy()
    
    }
}
