use super::super::server::generator::graph::Graph;


use super::super::server::generator::operator::OperatorTrait;


pub enum EnvMessage
{
	Startclient,
	PopClient(Box<Vec<Graph>>),
	OperatorTraitEnv(Vec<Box<OperatorTrait + Send>>),
	RepetitionsEnv(u32),
	EndPopClient

}
