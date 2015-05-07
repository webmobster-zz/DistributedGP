use super::server::generator::graph::Graph;

use super::server::generator::operator::OperatorTrait;


pub enum ServerMessage
{
	Start,
	PopVec(Box<Vec<Graph>>),
	OperatorTraitClient(Vec<Box<OperatorTrait + Send>>),
	
	RepetitionsClient(u32),
	EndPop

}
