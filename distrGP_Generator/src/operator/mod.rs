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
	op: fn(&mut Arc<Mutex<GlobalState>>, &mut LocalState) -> bool,
	parts: Option<Vec<[u64;2]>>,
	sucessors: u8


}

impl Operator
{

	pub fn new(op:  fn(&mut Arc<Mutex<GlobalState>>, &mut LocalState) -> bool, parts:Option<Vec<[u64;2]>>, sucessors: u8) -> Operator
	{
		Operator{ op: op, parts: parts, sucessors: sucessors}

	}
	pub fn get_sucessors(&self) -> u8
	{
		self.sucessors

	}
	pub fn call(&self, global: &mut Arc<Mutex<GlobalState>>, local: &mut LocalState)->bool
	{
		let func= self.op;
		func(global,local)

	}


}

impl Clone for Operator
{

	fn clone(&self) -> Operator
	{
		Operator{op: self.op, parts: self.parts.clone(),sucessors: self.sucessors}

	}

}

//#[derive(Debug,Clone)]
pub type OperatorMap = HashMap<[u64;2],Operator>;

pub trait RandomKey
{
	fn random_key<R: Rng>(&self,rng: &mut R) ->[u64;2];

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

}









