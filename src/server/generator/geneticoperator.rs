use super::Graph;
use super::Selector;


pub trait GeneticOperator
{
	fn operate(&self, pop: &Vec<Graph>, selector: &Box<Selector>) -> Vec<Graph>;
	fn get_probability(&self) -> f32;
	fn get_copy(&self) -> Box<GeneticOperator>;
}


impl Clone for Box<GeneticOperator>
{
    fn clone(&self) -> Box<GeneticOperator>
    {
        self.get_copy()
    
    }
}
