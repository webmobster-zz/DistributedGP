use super::BiChannel;
use super::Graph;

use std::cmp::Ordering;
use std::cmp::Ordering::{Less,Equal,Greater};
use std::sync::{Arc, Mutex};


pub enum StateIO
{
	Start,
	Data(u64),
	Fitness(u64),
	SizeGraph(u64),
	SizeVec(u64),
	Life(u64),
	Done

}

pub struct GlobalState
{
	//Persistant after evaluation and after selection and mutable by multiple threads
	pub vec: Arc<Mutex<Vec<u64>>>,
	pub graph: Arc<Mutex<Graph>>,
	pub life: Option<Arc<Mutex<u64>>>,

	//mutable by multiple threads
	pub comm: Option<Arc<Mutex<BiChannel<StateIO>>>>,
	pub thread_count: Option<Arc<Mutex<u64>>>,


	//Persistant after evaluation but not after selection
	pub fitness: Option<Arc<Mutex<u64>>>,
	
}

impl Clone for GlobalState
{
	fn clone(&self) ->GlobalState
	{
		GlobalState{vec: self.vec.clone(), comm: self.comm.clone(), life: self.life.clone(),graph: self.graph.clone(),thread_count: self.thread_count.clone(),fitness: self.fitness.clone()}
	}

}

impl GlobalState
{


	pub fn new(memory: Vec<u64>, graph: Graph) -> GlobalState
	{

		GlobalState{vec: Arc::new(Mutex::new(memory)), comm: None, life: None,graph:  Arc::new(Mutex::new(graph)), fitness: None, thread_count: None}

	}
	//drops input, output and threadcount
	pub fn unique_copy(&mut self) -> GlobalState
	{

		let veclock =self.vec.lock().unwrap();
		let graphlock =self.graph.lock().unwrap();
		let life =self.life.clone().unwrap();
		let lifelock= life.lock().unwrap();

		let fitness =self.fitness.clone().unwrap();
		let fitnesslock= fitness.lock().unwrap();
		GlobalState{vec: Arc::new(Mutex::new(veclock.clone())), comm: None, life: Some(Arc::new(Mutex::new(lifelock.clone()))),graph:  Arc::new(Mutex::new(graphlock.clone())), thread_count: None,fitness: Some(Arc::new(Mutex::new(fitnesslock.clone())))}
	}

	pub fn initialize(&mut self,life: u64, ) -> BiChannel<StateIO>
	{

		let (end_one,end_two) = BiChannel::new();
		self.comm= Some(Arc::new(Mutex::new(end_one)));
		self.life=Some(Arc::new(Mutex::new(life)));
		self.thread_count = Some(Arc::new(Mutex::new(0)));
		self.fitness=Some(Arc::new(Mutex::new(0)));
		end_two
	}
	pub fn unique_graphvec_copy(&self) -> (Graph,Vec<u64>)
	{
		let veclock =self.vec.lock().unwrap();
		let graphlock =self.graph.lock().unwrap();
		(graphlock.clone(),veclock.clone())

	}
	pub fn get_fitness(&self) -> u64
	{
		let fitness = self.fitness.clone().unwrap();
		let lockfit =  *fitness.lock().unwrap();
		lockfit
	}

}


//These are all to allow sorting based on fitness

//This is probably a bad idea


//Need to remeber to not deadlock if self and other are the same
impl Eq for GlobalState
{

}

impl PartialOrd for GlobalState
{

	fn partial_cmp(&self, other: &GlobalState) -> Option<Ordering>
	{
		let lockself: u64;
		let lockother: u64;
		{
			let fitness_self = self.fitness.clone().unwrap();
			lockself =  *fitness_self.lock().unwrap();
			 
		}
		{
			let fitness_other = other.fitness.clone().unwrap();
			lockother =  *fitness_other.lock().unwrap();
		}

		if lockself < lockother { Some(Less) }
    		else if lockself > lockother { Some(Greater) }
    		else { Some(Equal) }

	}


}



impl PartialEq for GlobalState
{

	fn  eq(&self, other: &GlobalState) -> bool
	{
		let lockself: u64;
		let lockother: u64;
		{
			let fitness_self = self.fitness.clone().unwrap();
			lockself =  *fitness_self.lock().unwrap();
			 
		}
		{
			let fitness_other = other.fitness.clone().unwrap();
			lockother =  *fitness_other.lock().unwrap();
		}

		if lockself == lockother { true }
    		else {false}

	}


}

impl Ord for GlobalState
{

	fn cmp(&self, other: &GlobalState) -> Ordering
	{


		let lockself: u64;
		let lockother: u64;
		{
			let fitness_self = self.fitness.clone().unwrap();
			lockself =  *fitness_self.lock().unwrap();
			 
		}
		{
			let fitness_other = other.fitness.clone().unwrap();
			lockother =  *fitness_other.lock().unwrap();
		}

		if lockself < lockother { Less }
    		else if lockself > lockother { Greater }
    		else { Equal }

	}


}

#[derive(Copy)]
pub struct LocalState
{
	pub node: Option<usize>,
	pub array: [u64;1000],

	pub vec_pointer: usize,
	pub array_pointer: usize,
	pub general_pointer: u64

	
}

impl LocalState
{
	pub fn new() -> LocalState
	{

		LocalState{node: Some(0), array: [0;1000],array_pointer: 0, general_pointer: 0, vec_pointer: 0}

	}

}

impl Clone for LocalState
{
	fn clone(&self) -> LocalState
	{
		LocalState{node: self.node, array: self.array ,array_pointer: self.array_pointer, general_pointer: self.general_pointer, vec_pointer: self.vec_pointer}
	}

}


