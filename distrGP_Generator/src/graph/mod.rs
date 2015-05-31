//Holds the algorithms type

extern crate rand; 

use super::operator::Operator;

use  std::cmp::min;
use  std::cmp::max;

use std::cmp::Ordering;
use std::cmp::Ordering::{Less,Equal,Greater};


#[derive(Debug,Clone)]
pub struct Graph
{
	     list: Vec<Node>,
	     fitness: Option<u32>,
	     life: Option<u32>,
	     perfect: Option<bool>,
}

#[derive(Clone,Debug)]
pub struct Node( pub Operator,pub Option<usize>,pub Option<usize>);



//These are all to allow sorting based on fitness

//This is probably a bad idea
impl Eq for Graph
{

}

impl PartialOrd for Graph
{

	fn partial_cmp(&self, other: &Graph) -> Option<Ordering>
	{
		if self.fitness.unwrap() < other.fitness.unwrap() { Some(Less) }
    		else if self.fitness.unwrap() > other.fitness.unwrap() { Some(Greater) }
    		else { Some(Equal) }

	}


}



impl PartialEq for Graph
{

	fn  eq(&self, other: &Graph) -> bool
	{
		if self.fitness.unwrap() == other.fitness.unwrap() { true }
    		else {false}

	}


}

impl Ord for Graph
{

	fn cmp(&self, other: &Graph) -> Ordering
	{
		if self.fitness.unwrap() < other.fitness.unwrap() { Less }
    		else if self.fitness.unwrap() > other.fitness.unwrap() { Greater }
    		else { Equal }

	}


}






impl Graph
{



	pub fn empty_graph() -> Graph
	{
		Graph{list: Vec::new(), fitness: None, life: None,perfect: None}
	}

	


	



	pub fn get_sucessor_index(&self,  mut index: usize) -> (Option<usize>,Option<usize>)
	{
		index = index % self.list.len();

		match self.list[index]
		{
			Node(_, suc1,suc2) => (suc1,suc2),

		}


	}



	pub fn get_operator(&self, mut index: usize) -> Operator
	{
		index = index % self.list.len();

		
		let op =match self.list[index]
		{
			Node(ref op, _,_) => op,
		};
		op.clone()

	}

	pub fn set_fitness(&mut self, fitness: u32)
	{
		self.fitness = Some(fitness);

	}
	pub fn get_fitness(& self) -> u32
	{
		self.fitness.unwrap()

	}

	pub fn set_perfect(&mut self, perfect: bool)
	{
		self.perfect = Some(perfect);

	}
	pub fn get_perfect(& self) -> bool
	{
		self.perfect.unwrap()

	}

	pub fn set_life(&mut self, life: u32)
	{
		self.life = Some(life);

	}
	pub fn get_life(& self) -> Option<u32>
	{
		self.life

	}

	pub fn get_size(& self) -> usize
	{
		self.list.len()

	}


	pub fn get_slice<'a>(&'a self,first :usize, second: usize) -> &'a [Node]
	{
		let higher = max(first,second);
		let lower = min(first,second);
		&self.list[lower..higher]


	}

	pub fn replace_slice(&mut self,start :usize, slice: &[Node])
	{

		let mut x=start;
		for i in 0 .. slice.len()
		{	

			x = x +1;

			if  x < self.list.len()
			{
				self.list[x]=slice[i].clone();
			}
			else if self.list.len() ==x
			{

				self.list.push(slice[i].clone());
			}
			else
			{
				panic!("error in logic somewhere");
			}

		}



	}




	

	pub fn get_labeled_nodes(& self) -> Vec<String>
	{
		let mut labels  = Vec::new();

		let s = format!("{{begining:{}}}",self.get_operator(0).get_refnum());
		labels.push(s);

		for i in (1 .. self.list.len())
		{

			let s = format!("{{{}}}",self.get_operator(i).get_refnum());
			labels.push(s);


		}
		labels

	}

	pub fn get_edges(& self) -> Vec<(usize,usize,Option<bool>)>
	{
		let mut edges  = Vec::new();
		for i in 0 .. self.list.len()
		{

			let (suc1,suc2) = self.get_sucessor_index(i);


			if suc2 != None
			{
				edges.push((i, (suc1.unwrap())  % self.list.len() ,Some(true)));
				edges.push((i, (suc2.unwrap())  % self.list.len() ,Some(false)));

			}else if suc1 != None
			{
				edges.push((i, (suc1.unwrap())  % self.list.len() ,None));

			}
			

			

			


		}
		edges

	}

	pub fn set_node(&mut self, index: usize, node: Node)
	{
		self.list[index] = node;
	}

	pub fn get_node(&self,index: usize ) -> Node
	{
		self.list[index].clone()

	}

	pub fn remove_node(&mut self,index: usize )
	{
		self.list.remove(index);

	}


	pub fn add_to_end(&mut self, node:  Node)
	{
		self.list.push(node);

	}






}


