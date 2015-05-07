//Holds the algorithms type

extern crate rand; 

use super::operator::Operator;


use  std::cmp::min;
use  std::cmp::max;


use self::rand::distributions::{IndependentSample, Range};


use std::cmp::Ordering;
use std::cmp::Ordering::{Less,Equal,Greater};


use std::collections::HashMap;
use std::collections::VecDeque;

pub mod visualize;

#[derive(Debug,Clone)]
pub struct Graph
{
	     pub list: Vec<Node>,
	     loose_ends: Vec<usize>,
	     fitness: Option<u32>,
	     parent1: Option<Box<Graph>>,
	     parent2: Option<Box<Graph>>,
	     pub life: u32,
	     perfect: Option<bool>
}

#[derive(Clone,Debug)]
pub struct Node(   pub Operator, pub isize,pub isize, pub isize);


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



	pub fn empty_graph(fitness:Option<u32>, life: u32) -> Graph
	{
		Graph{list: Vec::new(), loose_ends: Vec::new(), fitness: fitness, life: life,perfect: None, parent1: None, parent2: None}
	}

	pub fn grow_graph(operatorpointers: &Vec<Operator>, end_operators: &Vec<u32>, initial_size: u32, life: u32) -> Graph
	{

		if operatorpointers.len() ==0
		{
			panic!("no operators defined");

		}

		if end_operators.len() ==0
		{
			panic!("no end point operators defined");

		}


		let mut new_graph = Graph{list: Vec::with_capacity(initial_size as usize), loose_ends: Vec::new(), perfect: None,fitness: None, life: life, parent1: None, parent2: None};


		let operator_count = Range::new(0, operatorpointers.len());

		//fast but bad
	   	let mut rng = rand::weak_rng();


		//intial node
		let operator = operatorpointers[operator_count.ind_sample(&mut rng)].clone();

		new_graph.add_to_end_node(operator);


		//Grow behaviour comes from here
		loop
		{	



			while new_graph.list.len() < initial_size as usize
			{


				//no further succesors possible
				if new_graph.loose_ends.len() == 0
				{
					break;
				}



				//deep behaviour
			

				//get loose end
				let working_index = new_graph.loose_ends.pop().unwrap();
			

				let node = new_graph.list[working_index].clone();

				let Node(op,_,_,_) = node;

				let end = new_graph.list.len();
			
				if op.sucessors ==3
				{
					panic!("unimplemented feature");

				}
				
				else if op.sucessors ==2
				{
					//replace unlinked node with node with links
					new_graph.list[working_index] = Node(op,end as isize,(end +1) as isize,-1);

					//add new node with unfilled sucessors to the ends
					new_graph.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());
					new_graph.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());

				}
				else if op.sucessors ==1
				{

					new_graph.list[working_index] = Node(op,end as isize,-1,-1);

					//add new node with unfilled sucessors to the ends
					new_graph.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());

				}
				else if op.sucessors ==0
				{
					
					//do nothing, as the Node should have unconnected sucessors already
					//hopefully this will get compiled away, usefull for code readability sakes


				}



			}


			//CLEAR UP DANGLING NODES
			

			//no further succesors possible
			if new_graph.loose_ends.len() == 0
			{
				//clear large empty vec
				new_graph.loose_ends = Vec::new();
				break;
			}


			let end_operator_count = Range::new(0, end_operators.len());
			
			//get a random index from the end operator list, which is used to get an operator from the operator list
			//SAME OPERATOR EVERY TIME FIX THIS!
			let operator = operatorpointers[end_operators[end_operator_count.ind_sample(&mut rng)] as usize].clone();

			let working_index = new_graph.loose_ends.pop().unwrap();
			
			let node = new_graph.list[working_index].clone();

			let Node(op,_,_,_) = node;

			let end = new_graph.list.len();


			if op.sucessors ==3
			{
				panic!("unimplemented feature");

			}
			
			else if op.sucessors ==2
			{
				//replace unlinked node with node with links
				new_graph.list[working_index] = Node(op,end as isize,(end +1) as isize,-1);

				//add new node with unfilled sucessors to the ends
				new_graph.add_to_end_node(operator.clone());
				new_graph.add_to_end_node(operator.clone());

			}
			else if op.sucessors ==1
			{
				//replace unlinked node with node with link
				new_graph.list[working_index] = Node(op,end as isize,-1,-1);

				//add new node with unfilled sucessors to the ends
				new_graph.add_to_end_node(operator);

			}
			else if op.sucessors ==0
			{

				//do nothing, as the Node should have unconnected sucessors already
				//hopefully this will get compiled away, usefull for code readability sakes



			}

		}
		new_graph

	}


	pub fn grow_new_subtree(&mut self,operatorpointers: &Vec<Operator>,end_operators: &Vec<u32>,index: usize, targetsize: usize)
	{

		//THIS DOESNT CLEAN UP FREE NODES
		let operator_count = Range::new(0, operatorpointers.len());

		//fast but bad
	   	let mut rng = rand::weak_rng();


		//intial node
		let operator = operatorpointers[operator_count.ind_sample(&mut rng)].clone();

		let new_node = Node(operator, -1,-1,-1);

		self.list[index]=new_node;

		self.loose_ends.push(index);



		//Grow behaviour comes from here
		loop
		{	



			while self.list.len() < targetsize
			{


				//no further succesors possible
				if self.loose_ends.len() == 0
				{
					break;
				}



				//deep behaviour
			

				//get loose end
				let working_index = self.loose_ends.pop().unwrap();
			

				let node = self.list[working_index].clone();

				let Node(op,_,_,_) = node;

				let end = self.list.len();
			
				if op.sucessors ==3
				{
					panic!("unimplemented feature");

				}
				
				else if op.sucessors ==2
				{
					//replace unlinked node with node with links
					self.list[working_index] = Node(op,end as isize,(end +1) as isize,-1);

					//add new node with unfilled sucessors to the ends
					self.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());
					self.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());

				}
				else if op.sucessors ==1
				{

					self.list[working_index] = Node(op,end as isize,-1,-1);

					//add new node with unfilled sucessors to the ends
					self.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());

				}
				else if op.sucessors ==0
				{
					
					//do nothing, as the Node should have unconnected sucessors already
					//hopefully this will get compiled away, usefull for code readability sakes


				}



			}


			//CLEAR UP DANGLING NODES
			

			//no further succesors possible
			if self.loose_ends.len() == 0
			{
				//clear large empty vec
				self.loose_ends = Vec::new();
				break;
			}


			let end_operator_count = Range::new(0, end_operators.len());
			
			//get a random index from the end operator list, which is used to get an operator from the operator list
			//SAME OPERATOR EVERY TIME FIX THIS!
			let operator = operatorpointers[end_operators[end_operator_count.ind_sample(&mut rng)] as usize].clone();

			let working_index = self.loose_ends.pop().unwrap();
			
			let node = self.list[working_index].clone();

			let Node(op,_,_,_) = node;

			let end = self.list.len();

			if op.sucessors ==3
			{
				panic!("unimplemented feature");

			}
			
			else if op.sucessors ==2
			{
				//replace unlinked node with node with links
				self.list[working_index] = Node(op,end as isize,(end +1) as isize,-1);

				//add new node with unfilled sucessors to the ends
				self.add_to_end_node(operator.clone());
				self.add_to_end_node(operator.clone());

			}
			else if op.sucessors ==1
			{
				//replace unlinked node with node with link
				self.list[working_index] = Node(op,end as isize,-1,-1);

				//add new node with unfilled sucessors to the ends
				self.add_to_end_node(operator);

			}
			else if op.sucessors ==0
			{

				//do nothing, as the Node should have unconnected sucessors already
				//hopefully this will get compiled away, usefull for code readability sakes



			}

		}





	}




	fn add_to_end_node(&mut self, operator:   Operator) 
	{

		let new_node = Node(operator, -1,-1,-1);
		let size = self.list.len();
		self.list.push(new_node);

		self.loose_ends.push(size);

	}


	pub fn remove_parents(&mut self)
	{
		self.parent1 = None; self.parent2 =None;

	}
	pub fn get_parents(&self) -> (Option<Box<Graph>>,Option<Box<Graph>>)
	{
		(self.parent1.clone(),self.parent2.clone())

	}

	pub fn add_parents(&mut self, parent1: Option<Graph>, parent2: Option<Graph>)
	{
		if parent1.is_some() {self.parent1 = Some(box parent1.unwrap());}
		if parent2.is_some() {self.parent2 = Some(box parent2.unwrap());}

	}

	pub fn get_sucessor_index(&self,  mut index: usize) -> (isize,isize,isize)
	{
		index = index % self.list.len();

		match self.list[index]
		{
			Node(_, suc1,suc2,suc3) => (suc1,suc2,suc3),

		}


	}



	pub fn get_operator(&self, mut index: usize) -> Operator
	{
		index = index % self.list.len();

		
		let op =match self.list[index]
		{
			Node(ref op, _,_,_) => op,
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


	pub fn replace_subtree(&mut self,start_replace :usize,start_get: usize, graph: &Graph)
	{
		let mut unfinished_nodes = VecDeque::new();

		let mut index_map: HashMap<isize,isize> = HashMap::new();



		//possibly more efficient way to do this
		index_map.insert(start_get as isize,start_replace as isize);

		unfinished_nodes.push_back((start_get as isize,start_replace as isize));
		




		//increment by one cause position 0 is needs to be allocated to the first node
		let mut last_used_position = self.list.len() as isize;

		while !unfinished_nodes.is_empty()
		{



			let (current_get_index,current_put_index)= match unfinished_nodes.pop_front()
			{
				Some(x) => x,
				None => panic!("should never happen"),

			};



			//does the current node exist, or is it getting put on the end
			let increase_graph_size;
			

			//should only occur in loops?
			if current_put_index< (self.list.len() as isize)
			{
				//panic!("loop");
				increase_graph_size=false;
			}
			else if(self.list.len() as isize) ==current_put_index
			{

				increase_graph_size=true;
				//last_used_position = last_used_position +1;
			}
			else
			{
				panic!("error in logic somewhere");
			}





			//make sure wrapping is accounted for
			//removed wrapping 4 testing purposes  % old.list.len()
			let mut current_node =graph.list[(current_get_index as usize) % graph.list.len()].clone();
			



			let Node(op, suc1,suc2,_) = current_node.clone();


		

			//THE END LOGIC should be fixed
			if op.sucessors ==3
			{
				panic!("unimplemented feature");

			}
			else if op.sucessors ==2
			{
				

					//problem seems to be here suc1s sucessorts will pop before suc2 does
					if !index_map.contains_key(&suc1) && !index_map.contains_key(&suc2)
					{
						//panic!("unimplemented feature");

						current_node = Node(op,last_used_position,(last_used_position +1),-1);
						
						
						
						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;


						index_map.insert(suc2,last_used_position);
						unfinished_nodes.push_back((suc2,last_used_position));
						last_used_position = last_used_position +1;

					}
					//handle loops
					else if index_map.contains_key(&suc1) && index_map.contains_key(&suc2)
					{
						//panic!("loop");
						current_node = Node(op,*index_map.get(&suc1).unwrap(),*index_map.get(&suc2).unwrap(),-1);

	
					}
					else if index_map.contains_key(&suc1)
					{
						//panic!("loop");
						current_node = Node(op,*index_map.get(&suc1).unwrap(),last_used_position,-1);
						
						index_map.insert(suc2,last_used_position);
						unfinished_nodes.push_back((suc2,last_used_position));
						last_used_position = last_used_position +1;

					}
					else
					{
						//panic!("loop");
						current_node = Node(op,last_used_position,*index_map.get(&suc2).unwrap(),-1);

						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;

					}
				


			}
			else if op.sucessors ==1
			{
					if !index_map.contains_key(&suc1)
					{


						current_node = Node(op,last_used_position,-1,-1);

						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;
					}
					else
					{
						//panic!("loop");
						current_node = Node(op,*index_map.get(&suc1).unwrap(),-1,-1);

					}
				


			}
			else if op.sucessors ==0
			{





			}


			//replace or grow the list
			if !increase_graph_size
			{
				self.list[current_put_index as usize]=current_node.clone();
			}
			else
			{

				self.list.push(current_node.clone());
			}

			//println!("working end");

		


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

			let (suc1,suc2,_) = self.get_sucessor_index(i);


			if suc2 != -1
			{
				edges.push((i, (suc1 as usize)  % self.list.len() ,Some(true)));
				edges.push((i, (suc2 as usize)  % self.list.len() ,Some(false)));

			}else if suc1 != -1
			{
				edges.push((i, (suc1 as usize)  % self.list.len() ,None));

			}
			

			

			


		}
		edges

	}

	pub fn remove_node(&mut self,index: usize )
	{
		self.list.remove(index);

	}


	//Havent checked this at all, just used a modified version of the replace subtree code
	//The method of 1000 off by 1 errors
	pub fn clean(&mut self)
	{
		let mut unfinished_nodes = VecDeque::new();

		let mut index_map: HashMap<isize,isize> = HashMap::new();


		let start_get=0;
		let start_replace=0;

		//possibly more efficient way to do this
		index_map.insert(start_get as isize,start_replace as isize);

		unfinished_nodes.push_back((start_get as isize,start_replace as isize));
		

		//probably best not to write to the same data structure we are reading
		let old = self.clone();

		

		//keeps track of allocated positions

		*self = Graph::empty_graph(old.fitness,old.life);


		//increment by one cause position 0 is needs to be allocated to the first node
		let mut last_used_position = self.list.len() as isize +1;

		while !unfinished_nodes.is_empty()
		{



			let (current_get_index,current_put_index)= match unfinished_nodes.pop_front()
			{
				Some(x) => x,
				None => panic!("should never happen"),

			};



			//does the current node exist, or is it getting put on the end
			let increase_graph_size;
			

			//should only occur in loops?
			if current_put_index< (self.list.len() as isize)
			{
				//panic!("loop");
				increase_graph_size=false;
			}
			else if(self.list.len() as isize) ==current_put_index
			{

				increase_graph_size=true;
				//last_used_position = last_used_position +1;
			}
			else
			{
				panic!("error in logic somewhere");
			}





			//make sure wrapping is accounted for
			//removed wrapping 4 testing purposes  % old.list.len()
			let mut current_node =old.list[(current_get_index as usize) % old.list.len() ].clone();
			



			let Node(op, suc1,suc2,_) = current_node.clone();


		

			//THE END LOGIC should be fixed
			if op.sucessors ==3
			{
				panic!("unimplemented feature");

			}
			else if op.sucessors ==2
			{
				

					//problem seems to be here suc1s sucessorts will pop before suc2 does
					if !index_map.contains_key(&suc1) && !index_map.contains_key(&suc2)
					{
						//panic!("unimplemented feature");

						current_node = Node(op,last_used_position,(last_used_position +1),-1);
						
						
						
						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;


						index_map.insert(suc2,last_used_position);
						unfinished_nodes.push_back((suc2,last_used_position));
						last_used_position = last_used_position +1;

					}
					//handle loops
					else if index_map.contains_key(&suc1) && index_map.contains_key(&suc2)
					{
						//panic!("loop");
						current_node = Node(op,*index_map.get(&suc1).unwrap(),*index_map.get(&suc2).unwrap(),-1);

	
					}
					else if index_map.contains_key(&suc1)
					{
						//panic!("loop");
						current_node = Node(op,*index_map.get(&suc1).unwrap(),last_used_position,-1);
						
						index_map.insert(suc2,last_used_position);
						unfinished_nodes.push_back((suc2,last_used_position));
						last_used_position = last_used_position +1;

					}
					else
					{
						//panic!("loop");
						current_node = Node(op,last_used_position,*index_map.get(&suc2).unwrap(),-1);

						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;

					}
				


			}
			else if op.sucessors ==1
			{
					if !index_map.contains_key(&suc1)
					{


						current_node = Node(op,last_used_position,-1,-1);

						index_map.insert(suc1,last_used_position);
						unfinished_nodes.push_back((suc1,last_used_position));
						last_used_position = last_used_position +1;
					}
					else
					{
						//panic!("loop");
						current_node = Node(op,*index_map.get(&suc1).unwrap(),-1,-1);

					}
				


			}
			else if op.sucessors ==0
			{





			}


			//replace or grow the list
			if !increase_graph_size
			{
				self.list[current_put_index as usize]=current_node.clone();
			}
			else
			{

				self.list.push(current_node.clone());
			}

			//println!("working end");



		}


	}






}


