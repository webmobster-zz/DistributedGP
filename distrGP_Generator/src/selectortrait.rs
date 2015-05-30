use super::graph::Graph;


pub trait Selector
{
	fn select(&self, pop: &Vec<Graph>) -> Vec<Graph>;
	fn get_copy(&self) -> Box<Selector>;
}


impl Clone for Box<Selector>
{
    fn clone(&self) -> Box<Selector>
    {
        self.get_copy()
    
    }
}
