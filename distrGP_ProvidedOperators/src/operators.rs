extern crate distrGP_Generator;
extern crate rand;

use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::Operator;
use self::distrGP_Generator::GlobalState;
use self::distrGP_Generator::LocalState;
use self::distrGP_Generator::StateIO;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex};
use self::rand::Rng;

pub fn load_operators(map: &mut OperatorMap)
{

	let mut rng = rand::thread_rng();
	map.insert([rng.gen::<u64>(); 2],Operator::new(incr_pointer,None,1,10));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(decr_pointer,None,1));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(incr_array_pointer,None,2));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(decr_array_pointer,None,2));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(get_input,None,2));
	map.insert([rng.gen::<u64>(); 2],Operator::new(send_output,None,1,10));

	map.insert([rng.gen::<u64>(); 2],Operator::new(end,None,0,10));

}

fn get_input(global: &mut GlobalState, local: &mut LocalState) -> bool
{
	let input = global.input.clone().unwrap();
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
	let output = global.output.clone().unwrap();
	let lock = output.lock().unwrap();
	match lock.send(StateIO::Data(local.general_pointer))
	{
		Ok(_) => {true},
		_=> panic!("Dropped Comms")

	}

}


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

	if local.array_pointer < 0
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

