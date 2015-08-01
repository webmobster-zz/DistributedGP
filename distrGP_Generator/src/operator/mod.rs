use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};
use super::GlobalState;
use super::LocalState;
use rand::Rng;
use std::str::FromStr;

pub mod operator_compiler;

#[derive(Clone,Debug,Copy,Eq,PartialEq,Hash)]
pub struct UUID {x: [u64;2]}
impl FromStr for UUID {
	type Err = ();

            #[inline]
            #[allow(deprecated)]
            fn from_str(src: &str) -> Result<Self, ()> {
		let mut split = src.split(",");
                let part1 = split.next().unwrap();
		let part2 = split.next().unwrap();
		Ok(UUID{x:[part1.parse::<u64>().unwrap(),part2.parse::<u64>().unwrap()]})
		
            }
}



#[derive(Clone,Debug,Copy)]
pub enum SpecialOperator
{
	None,
	NewThread,

}

//#[derive(Clone,Debug)]
pub struct Operator
{
	op: fn(&mut GlobalState, &mut LocalState) -> bool,
	parts: Option<[u64;2]>,
	sucessors: u8,
	cost: u64,
	special: SpecialOperator


}

impl Operator
{

	pub fn new(op:  fn(&mut GlobalState, &mut LocalState) -> bool, parts:Option<[u64;2]>, sucessors: u8, cost: u64, special: SpecialOperator) -> Operator
	{
		Operator{ op: op, parts: parts, sucessors: sucessors, cost: cost, special: special}

	}
	pub fn get_sucessors(&self) -> u8
	{
		self.sucessors

	}
	pub fn get_base_cost(&self) -> u64
	{
		self.cost

	}
	pub fn get_special(&self) -> SpecialOperator
	{
		self.special

	}
	pub fn call(&self, global: &mut GlobalState, local: &mut LocalState)->bool
	{
		let func= self.op;
		func(global,local)

	}


}

impl Clone for Operator
{

	fn clone(&self) -> Operator
	{
		Operator{op: self.op, parts: self.parts.clone(),sucessors: self.sucessors, cost: self.cost, special: self.special }

	}

}

//#[derive(Debug,Clone)]
pub type OperatorMap = HashMap<UUID,Operator>;

pub trait RandomKey
{
	fn random_key<R: Rng>(&self,rng: &mut R) ->UUID;
	fn random_key_with_successors<R: Rng>(&self,rng: &mut R, suc: u8) ->Result<UUID,()>;
}


impl RandomKey for OperatorMap
{

	fn random_key<R: Rng>(&self,rng: &mut R) ->UUID
	{


		let operator_count = Range::new(0, self.len());
		let rand = operator_count.ind_sample(rng);
		let mut key;
		let mut x =0;
		for i in self.keys()
		{
			if x==rand
			{
				key=*i;
				return key;
			}

			x+=1;
		}
		panic!("failed to get a random key");


	}

	fn random_key_with_successors<R: Rng>(&self,rng: &mut R, suc: u8) ->Result<UUID,()>
	{

		let mut operators = Vec::new();

		for entry in self.iter()
		{
			let (hash,op)=entry;
			if op.get_sucessors() == suc
			{
				operators.push(*hash);
			}

		}
		if operators.len()==0
		{
			return Err(())

		}
		let operator_count = Range::new(0, operators.len());
		Ok(operators[operator_count.ind_sample(rng)])



	}


}









