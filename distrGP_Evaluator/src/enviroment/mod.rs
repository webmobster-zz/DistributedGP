extern crate alloc;
extern crate distrGP_Generator;


use super::server::ServerMessage;


use self::distrGP_Generator::Graph;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::Operator;
use self::distrGP_Generator::GlobalState;
use self::distrGP_Generator::LocalState;
use self::pool::ThreadPool;
use self::pool::GreenThreadData;

use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

mod pool;



pub fn init(send: SyncSender<ServerMessage>, receive: Receiver<ServerMessage>)
{ 


	
	loop
	{

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				ServerMessage::Start => {},
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
				
		}


		let op_trait;

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				ServerMessage::OperatorMap(x) => {op_trait = x},
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
				
		}






		// receive population

		let mut vec_pops;
		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				ServerMessage::PopVec(x) => vec_pops=x,
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
					

		};	

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				ServerMessage::EndPop => (),
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")

			
		};	

		println!("env got {} graphs",vec_pops.len());

		// run all populations and send fitnesses 
		run(&mut vec_pops,&op_trait, 12);

		assert!(send.send(ServerMessage::PopVec(vec_pops)).is_ok());

		assert!(send.send(ServerMessage::EndPop).is_ok());
	

		//loop back to beginning




	}
	
}
#[allow(unused_mut)]
fn run(pop: &mut Vec<Graph>, map: &OperatorMap,thread_count: u32)
{



	for i in 0 .. pop.len()
	{

		let working_graph = (&mut **pop).get_mut(i).unwrap();

		let (result,perfect) = iterate_over_entity(working_graph, map,thread_count as usize);

		working_graph.set_fitness(result);
		working_graph.set_perfect(perfect);

			

	}

}


//result, life
//actual method that iterates over an individual
fn iterate_over_entity(entity: &Graph, map: &OperatorMap, thead_num: usize) -> (u32,bool)
{


	let initial_global_state = GlobalState{vec: vec!(1), vec_pointer: 5};
	let initial_local_state = LocalState::new();




	
	let pool= Arc::new(Mutex::new(ThreadPool::new(thead_num)));



	let green= GreenThreadData::new(initial_global_state.clone(),initial_local_state, entity,map);
	    
	pool.lock().unwrap().execute(green,pool.clone());

	//Hacky
	while alloc::arc::strong_count(&pool) > 1
	{
		//std::thread::sleep_ms(100);
	}


	let mut life = entity.get_life().unwrap();


		

	let mut index: usize = 0;
		
 	(8,true)

}



