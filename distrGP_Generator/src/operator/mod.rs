extern crate rand;


use std::fmt;
use std::collections::HashMap;
use self::rand::distributions::{IndependentSample, Range};
use super::GlobalState;
use super::LocalState;
use rand::Rng;
use std::sync::{Arc, Mutex};

//#[derive(Clone,Debug)]
pub struct Operator
{
	op: fn(&mut GlobalState, &mut LocalState) -> bool,
	parts: Option<[u64;2]>,
	sucessors: u8,
	cost: u64


}

impl Operator
{

	pub fn new(op:  fn(&mut GlobalState, &mut LocalState) -> bool, parts:Option<[u64;2]>, sucessors: u8, cost: u64) -> Operator
	{
		Operator{ op: op, parts: parts, sucessors: sucessors, cost: cost}

	}
	pub fn get_sucessors(&self) -> u8
	{
		self.sucessors

	}
	pub fn get_base_cost(&self) -> u64
	{
		self.cost

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
		Operator{op: self.op, parts: self.parts.clone(),sucessors: self.sucessors, cost: self.cost}

	}

}

//#[derive(Debug,Clone)]
pub type OperatorMap = HashMap<[u64;2],Operator>;

pub trait RandomKey
{
	fn random_key<R: Rng>(&self,rng: &mut R) ->[u64;2];
	fn random_key_with_successors<R: Rng>(&self,rng: &mut R, suc: u8) ->Result<[u64;2],()>;
}


impl RandomKey for OperatorMap
{

	fn random_key<R: Rng>(&self,rng: &mut R) ->[u64;2]
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

	fn random_key_with_successors<R: Rng>(&self,rng: &mut R, suc: u8) ->Result<[u64;2],()>
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









