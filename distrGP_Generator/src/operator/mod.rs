use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};
use super::GlobalState;
use super::LocalState;
use rand::Rng;
use std::str::FromStr;
use std::fmt;

pub mod operator_compiler;

#[derive(Clone,Debug,Copy,Eq,PartialEq,Hash)]
pub struct UUID {pub x: [u64;2]}
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

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.x[0], self.x[1])
    }
}



#[derive(Clone,Debug,Copy)]
pub enum SpecialOperator
{
	None,
	NewThread

}

//Rational behind this is that it may be in the future compiled operators are stored in
//a database rather than in memory
pub struct MinifiedOperator
{
	op: fn(&mut GlobalState, &mut LocalState) -> bool,
	sucessors: u8,
	cost: u64,
	special: SpecialOperator
}

impl MinifiedOperator
{

	pub fn new(op:  fn(&mut GlobalState, &mut LocalState) -> bool, sucessors: u8, cost: u64, special: SpecialOperator) -> MinifiedOperator
	{
		MinifiedOperator{ op: op, sucessors: sucessors, cost: cost, special: special}

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

impl Clone for MinifiedOperator
{

	fn clone(&self) -> MinifiedOperator
	{
		MinifiedOperator{op: self.op,sucessors: self.sucessors, cost: self.cost, special: self.special }

	}

}

pub type OperatorMap = HashMap<UUID,MinifiedOperator>;

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
		let key;
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
