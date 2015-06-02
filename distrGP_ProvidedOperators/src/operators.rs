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
	map.insert([rng.gen::<u64>(); 2],Operator::new(incr_local_pointer,None,1));
	map.insert([rng.gen::<u64>(); 2],Operator::new(end,None,0));

}

fn incr_local_pointer(global: &mut Arc<Mutex<GlobalState>>, local: &mut LocalState) -> bool
{
	local.local_pointer+=1;
	//println!("lolol");
	true

}
fn end(global: &mut Arc<Mutex<GlobalState>>, local: &mut LocalState) -> bool
{
	//println!("woop");
	true

}

