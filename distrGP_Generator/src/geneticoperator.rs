use super::Graph;
use super::OperatorMap;

pub trait GeneticOperator
{
	fn operate(&self, operators: &mut OperatorMap,selector_closure: &Box<Fn() -> (Graph,Vec<u64>)>) -> Vec<(Graph,Vec<u64>)>;
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
