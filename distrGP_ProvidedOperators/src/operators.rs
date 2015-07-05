extern crate distrgp_generator;
extern crate rand;

use self::distrgp_generator::OperatorMap;
use self::distrgp_generator::Operator;
use self::distrgp_generator::SpecialOperator;
use self::distrgp_generator::GlobalState;
use self::distrgp_generator::LocalState;
use self::distrgp_generator::StateIO;

use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex};

use self::rand::Rng;

pub fn load_operators(map: &mut OperatorMap)
{

	let mut rng = rand::thread_rng();
	map.insert([0,1],Operator::new(incr_pointer,None,1,1,SpecialOperator::None));
	map.insert([0,2],Operator::new(decr_pointer,None,1,1,SpecialOperator::None));

	map.insert([0,3],Operator::new(incr_array_pointer,None,2,1,SpecialOperator::None));
	map.insert([0,4],Operator::new(decr_array_pointer,None,2,1,SpecialOperator::None));

	map.insert([0,5],Operator::new(if_zero,None,2,1,SpecialOperator::None));

	map.insert([0,6],Operator::new(pointer_to_local,None,1,1,SpecialOperator::None));
	map.insert([0,7],Operator::new(local_to_pointer,None,1,1,SpecialOperator::None));

	map.insert([0,9],Operator::new(get_input,None,2,1,SpecialOperator::None));
	map.insert([0,10],Operator::new(send_output,None,1,100,SpecialOperator::None));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(split,None,2,100000,SpecialOperator::NewThread));
	map.insert([0,11],Operator::new(end,None,0,1,SpecialOperator::None));

}

//Comparisons

fn if_zero(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	
	if local.general_pointer == 0
	{
		true

	}
	else
	{

		false
	}

}

//Memory and Util

fn pointer_to_local(global: &mut GlobalState, local: &mut LocalState) -> bool
{

	local.array[local.array_pointer] = local.general_pointer;
	true
}

fn local_to_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
{

	local.general_pointer= local.array[local.array_pointer];
	true
}

fn split(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	true
}

fn get_input(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	let input = global.comm.clone().unwrap();
	let lock = input.lock().unwrap();
	match lock.try_recv()
	{
		Ok(x) => match x
		{
			StateIO::Data(y) => {local.general_pointer=y;true},
			_=> panic!("Invalid Data"),
		},
		Err(e) => match e
		{
			TryRecvError::Empty=> {false},
			TryRecvError::Disconnected => panic!("Dropped Comms")

		}
	}	

}

fn send_output(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	let output = global.comm.clone().unwrap();
	let lock = output.lock().unwrap();
	match lock.send(StateIO::Data(local.general_pointer))
	{
		Ok(_) => {true},
		_=> panic!("Dropped Comms")

	}

}


//Mathematical and Logical


fn incr_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
{

		local.general_pointer = local.general_pointer.wrapping_add(1);
		true


}

fn decr_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
{

	local.general_pointer = local.general_pointer.wrapping_sub(1);
	true

}



fn incr_array_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	if local.array_pointer < local.array.len() -1
	{
		local.array_pointer+=1;
		true
	}
	else
	{
		false
	}

}

fn decr_array_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
{

	if local.array_pointer > 0
	{
		local.array_pointer-=1;
		true
	}
	else
	{
		false
	}
}


fn end(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	true
}

