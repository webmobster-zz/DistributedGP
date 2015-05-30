use std::fmt;

pub struct Operator {
    fptr_index: usize,

    //fix
    pub sucessors: u8,

}



pub trait OperatorTrait {



   
   // Static method signature; `Self` refers to the implementor type
   fn init(&mut self);

   // Instance methods, only signatures
   fn op(&mut self, usize) -> bool;

   fn fitness(&mut self) -> u32;

   fn get_random(&mut self) -> u32;

   fn secondary(&mut self,usize,&mut Vec<Option<(u32, u32)>>) -> (u32,bool);

   fn clone(& self) -> Box<OperatorTrait + Send>;

   fn init_state(& self) -> bool;
}


impl fmt::Debug for Operator
{

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
 	{
   			 write!(f, "Function Pointer :(ref: {0}, sucessors: {1})",self.fptr_index,self.sucessors)
  	}




}

impl Clone for Operator
{

	fn clone(&self) -> Operator
 	{
   			 Operator{fptr_index: self.fptr_index, sucessors: self.sucessors}
  	}




}




impl Operator
{

	pub fn new(fptr:usize,  sucessors: u8) -> Operator
	{
		Operator{fptr_index: fptr, sucessors: sucessors}

	}
	
	pub fn call(&self) -> usize
	{
		self.fptr_index


	}

	pub fn get_refnum(&self) -> usize
	{
		self.fptr_index

	}

	

}


