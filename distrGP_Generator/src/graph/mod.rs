//Holds the algorithms type

extern crate rand; 


use std::cmp::min;
use std::cmp::max;


#[derive(Debug,Clone)]
pub struct Graph
{
	     list: Vec<Node>
}

#[derive(Clone,Debug)]
pub struct Node( pub [u64;2],pub Option<usize>,pub Option<usize>);




impl Graph
{



	pub fn empty_graph() -> Graph
	{
		Graph{list: Vec::new()}
	}

	


	



	pub fn get_sucessor_index(&self,  mut index: usize) -> (Option<usize>,Option<usize>)
	{
		index = index % self.list.len();

		match self.list[index]
		{
			Node(_, suc1,suc2) => (suc1,suc2),

		}


	}



	pub fn get_operator(&self, mut index: usize) -> [u64;2]
	{
		index = index % self.list.len();

		
		let op =match self.list[index]
		{
			Node(ref op, _,_) => op,
		};
		op.clone()

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

		let s = format!("{{begining:{:?}}}",self.get_operator(0));
		labels.push(s);

		for i in (1 .. self.list.len())
		{

			let s = format!("{{{:?}}}",self.get_operator(i));
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


