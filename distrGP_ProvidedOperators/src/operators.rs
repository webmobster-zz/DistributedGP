extern crate distrGP_Generator;
extern crate rand;

use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::Operator;
use self::distrGP_Generator::GlobalState;
use self::distrGP_Generator::LocalState;
use std::sync::{Arc, Mutex};
use self::rand::Rng;

pub fn load_operators(map: &mut OperatorMap)
{

	let mut rng = rand::thread_rng();
	map.insert([rng.gen::<u64>(); 2],Operator::new(incr_local_pointer,None,2));
	map.insert([rng.gen::<u64>(); 2],Operator::new(decr_local_pointer,None,2));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(get_input,None,1));
	//map.insert([rng.gen::<u64>(); 2],Operator::new(send_output,None,1));

	map.insert([rng.gen::<u64>(); 2],Operator::new(end,None,0));

}
/*
fn get_input(global: &mut Arc<Mutex<GlobalState>>, local: &mut LocalState) -> bool
{
	let lock = global.lock().unwrap();
	match lock.input.try_recv()
	{
		Ok(x) => {local.general_pointer=x;true}
		Err(e) => {false}
	}	

}

fn send_output(global: &mut Arc<Mutex<GlobalState>>, local: &mut LocalState) -> bool
{
	let lock = global.lock().unwrap();
	match lock.output.send(local.general_pointer)
	{
		Ok(_) => {true}
		Err(_) => {false}
	}

}


*/

fn incr_local_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
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

fn decr_local_pointer(global: &mut GlobalState, local: &mut LocalState) -> bool
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

