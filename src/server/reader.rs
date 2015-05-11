extern crate rand;
//extern crate libc;


use super::generator::operator::Operator;

use super::generator::operator::OperatorTrait;

use std::collections::HashSet;
use super::generator::SelectionType;


use self::rand::distributions::{IndependentSample, Range};
use self::rand::Rng;




use std::u32;



pub struct ProblemDescription
{
	clientnum : u32,
	popcount: u32,
	initial_tree_size: u32,


	operatorpointers: Vec<Operator>,
	point_mutate_probability: f32,
	tree_mutate_probability: f32,
	crossover_probability: f32,
	flat_crossover_probability: f32,
	point_remove_probability: f32,
	clean_probability: f32,

	repetitions: u32,


	selection_type: SelectionType,

	operator_trait: Box<OperatorTrait  + Send>,

	life: u32
}



pub fn readfile() -> ProblemDescription
{
	let mut operatorindex: Vec<Operator> = Vec::new();
	let mut operatorpointers: Vec<CloneFixStruct> = Vec::new();
	
	//sucessors, reference number
	//reference number used for debugging/reading
	//Not strictly necessary for the running of the program
	operatorindex.push(Operator::new(0,1));
	operatorindex.push(Operator::new(1,2));
	operatorindex.push(Operator::new(2,0));


	operatorpointers.push(CloneFixStruct{fptr: move_forward});

	operatorpointers.push(CloneFixStruct{fptr: is_nine});

	operatorpointers.push(CloneFixStruct{fptr: return_index});


	

	ProblemDescription{ 
		//keep
		clientnum: num_cpus() as u32,
 		popcount: 50,
		initial_tree_size: 30,

		operatorpointers: operatorindex,
		repetitions: 50,
		selection_type: SelectionType::Tournament(2),
		life: 20000,

		//sort out
		point_mutate_probability: 0.1,
		tree_mutate_probability: 0.01,
		crossover_probability: 0.8,
		flat_crossover_probability: 0.001,
		point_remove_probability: 0.001,
		clean_probability: 0.088,



		
		operator_trait: Box::new ((OperatorStruct::new(

		operatorpointers,fitness,init,secondary

		)))

		}
}

impl ProblemDescription
{
	pub fn get_client_num(&self)-> u32
	{
		self.clientnum
	}

	pub fn get_popcount(&self)-> u32
	{
		self.popcount
	}
	pub fn get_operators(&self)-> Vec<Operator>
	{
		self.operatorpointers.clone()

	}

	pub fn get_end_operators(&self)-> Vec<u32>
	{

		let mut i = 0;
		let mut endlist= Vec::new();
		for op in self.operatorpointers.iter()
		{
			if op.sucessors == 0
			{
				endlist.push(i);
			}

			i = i+1;


		}
		endlist

	}
	pub fn get_operator_trait(&self) -> Box<OperatorTrait + Send>
	{
		self.operator_trait.clone()
		
	}


	pub fn get_point_mutate_probability(&self) -> f32
	{
		self.point_mutate_probability

	}
	pub fn get_tree_mutate_probability(&self) -> f32
	{
		self.tree_mutate_probability

	}
	pub fn get_crossover_probability(&self) -> f32
	{
		self.crossover_probability

	}
	pub fn get_flat_crossover_probability(&self) -> f32
	{
		self.flat_crossover_probability

	}
	pub fn get_point_remove_probability(&self) -> f32
	{
		self.point_remove_probability

	}
	pub fn get_clean_probability(&self) -> f32
	{
		self.clean_probability

	}


	pub fn get_repetitions(&self) -> u32
	{
		self.repetitions
	}

	pub fn get_selection_type(&self) -> SelectionType
	{
		self.selection_type.clone()

	}
	pub fn get_tree_size(&self) -> u32
	{
		self.initial_tree_size
	}
	pub fn get_life(&self) -> u32
	{
		self.life
	}


}

struct CloneFixStruct
{
	fptr: fn(&mut Vec<u32>,&mut usize) -> bool

}
impl Clone for CloneFixStruct
{

	fn clone(&self) -> CloneFixStruct
 	{
   			 CloneFixStruct{fptr: self.fptr}
  	}




}




pub struct OperatorStruct
{

	vec:   Vec<u32>,

	index: usize,

	init: bool,
	rand: u32,

	operators: Vec<CloneFixStruct>,
	fitnessfunction: fn(&mut Vec<u32>, &mut usize) -> u32,
	initfunction: fn() -> (Vec<u32>,usize),
	secondaryfunction: fn(&mut Vec<Option<(u32, u32)>>, usize) -> (u32,bool),
	
	

}

impl OperatorTrait for OperatorStruct
{


   // Static method signature; `Self` refers to the implementor type
   fn init(&mut self)
   {
	let mut rng = rand::thread_rng();


	let (vec,index)=init();

	self.rand = rng.gen::<u32>();
	self.vec=vec;
	self.index=index;
	self.init=true;

   }

   // Instance methods, only signatures
   fn op(&mut self, operator_number: usize) -> bool
   {

	(self.operators[operator_number].fptr)(&mut self.vec,&mut self.index)

   }

   fn fitness(&mut self) -> u32
   {
	(self.fitnessfunction)(&mut self.vec,&mut self.index)

   }

   fn get_random(&mut self) -> u32
   {
	self.rand

   }

   fn secondary(&mut self,length: usize, fitness_life_list: &mut Vec<Option<(u32, u32)>>) -> (u32,bool)
   {

	(self.secondaryfunction)(fitness_life_list,length)


   }
   fn init_state(&self) -> bool
   {

	self.init


   }
   fn clone(&self) -> Box<OperatorTrait + Send>
   {

		Box::new( OperatorStruct{
			vec:   self.vec.clone(),
			index: self.index,

			init: self.init,
			rand: self.rand,
			operators: self.operators.clone(),
			fitnessfunction: self.fitnessfunction,
			initfunction: self.initfunction,
			secondaryfunction: self.secondaryfunction,
		})


   }





}


impl OperatorStruct
{

  fn new(operators: Vec<CloneFixStruct>,fitnessfunction: fn(&mut Vec<u32>, &mut usize) -> u32,initfunction: fn() -> (Vec<u32>,usize),secondaryfunction: fn(&mut Vec<Option<(u32, u32)>>, usize) -> (u32,bool)) -> OperatorStruct
  {
    

	OperatorStruct
	{

		operators: operators,
		fitnessfunction: fitnessfunction,
		initfunction: initfunction,
		secondaryfunction: secondaryfunction,
		vec: Vec::new(),
		index: 0,
		init: false,
		rand:0
	}
	

  }




}


pub fn init() -> (Vec<u32>,usize)
{
	//[0,10)
	let between = Range::new(0u32, 4u32);
	let length = Range::new(1000u32, 2000u32);
	let mut rng = rand::weak_rng();

	//needs to have at least one starting value
	//update to random ints
	let mut vec =Vec::new();


	vec.push( between.ind_sample(&mut rng));

	for i in 0..length.ind_sample(&mut rng)
	{
		vec.push(between.ind_sample(&mut rng));
	}

	

	(vec,0)



}
#[allow(unused_variables)]
pub fn move_forward(input1: &mut Vec<u32>, input2: &mut usize) -> bool
{


		
		if *input2 == (input1.len()-1)
		{
			return true;

		}
		if *input2 > (input1.len()-1)
		{
			panic!("out of bounds");

		}
		*input2 = *input2+1;
		true
	
}

#[allow(unused_variables)]
pub fn is_nine(input1: &mut Vec<u32>, input2: &mut usize) -> bool
{
		
		if input1[*input2] ==3
		{
			return true
		}
		false

}


#[allow(unused_variables)]
pub fn return_index(input1: &mut Vec<u32>, input2: &mut usize) -> bool
{



		true
}






pub fn fitness(input1: &mut Vec<u32>, input2: &mut usize) -> u32
{

	
	
	if input1[*input2]==3
	{
		if *input2 > 0 && input1[*input2-1]==3
		{
			return 0
		}	
		return 10
	
	}
	20



}

pub fn secondary(fitness_life_list: &mut Vec<Option<(u32, u32)>>, length: usize) -> (u32,bool) 
{

	let mut fitness=0; let mut life=0;
 	for x in fitness_life_list.iter()
	{
		match *x
		{
			Some((f,l)) =>
				{
					fitness = fitness+ f;
					life = life + l;
				},
			None => return (u32::MAX,false)
		}

	
	}
	let mut perfect=false;
	if fitness==0 {perfect =true};


	(fitness + (length as f32 * 0.1) as u32,perfect)
}


pub fn num_cpus() -> usize {
    unsafe {
        return rust_get_num_cpus() as usize;
    }

    extern {
        fn rust_get_num_cpus() -> u64;
    }
}


