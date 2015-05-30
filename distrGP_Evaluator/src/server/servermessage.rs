use super::server::generator::graph::Graph;

use super::server::generator::operator::OperatorTrait;


pub enum ServerMessage
{
	Start,
	PopVec(Vec<Graph>),
	OperatorTrait(Vec<Box<OperatorTrait + Send>>),
	
	Repetitions(u32),
	EndPop

}
