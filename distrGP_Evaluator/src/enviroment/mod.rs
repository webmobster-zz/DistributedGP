extern crate alloc;
extern crate distrGP_Generator;


use super::server::ServerMessage;


use self::distrGP_Generator::Graph;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::Operator;
use self::distrGP_Generator::GlobalState;
use self::distrGP_Generator::LocalState;
use self::distrGP_Generator::StateIO;
use self::pool::ThreadPool;
use self::pool::GreenThreadData;

use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

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
fn run(pop: &mut Vec<GlobalState>, map: &OperatorMap,thread_count: u32)
{



	for i in 0 .. pop.len()
	{

		let working_graph = (&mut **pop).get_mut(i).unwrap();

		iterate_over_entity(working_graph, map,thread_count as usize);


			

	}

	for i in 0 .. pop.len()
	{

		let working_graph = (&mut **pop).get_mut(i).unwrap();


		let input = working_graph.input.clone().unwrap();
		let output = working_graph.output.clone().unwrap();

		let mut inputlock = input.lock().unwrap();
		let mut outputlock = output.lock().unwrap();
		let mut graphlock = working_graph.graph.lock().unwrap();
		let mut veclock = working_graph.vec.lock().unwrap();


		outputlock.send(StateIO::Done);		
		outputlock.send(StateIO::SizeVec(veclock.len() as u64));
		outputlock.send(StateIO::SizeGraph(graphlock.get_size() as u64));
		let mut fitness;
		loop
		{
			//clear the channel
			match inputlock.recv()
			{
				Ok(x) => {
					 	match x
						{
							StateIO::Data(_) => (),
							StateIO::Fitness(f) => {fitness =f; break},
							_ => panic!("Invalid Message")
						}
					},
				_ => panic!("Dropped receiver")
			}
		}
		working_graph.fitness = Some(fitness);

			

	}

}


//result, life
//actual method that iterates over an individual
fn iterate_over_entity(individual: &mut GlobalState, map: &OperatorMap, thead_num: usize)
{


	let initial_local_state = LocalState::new();

	


	
	let pool= Arc::new(Mutex::new(ThreadPool::new(thead_num)));



	let green= GreenThreadData::new(individual.clone(),initial_local_state,map);

	{
		let thread = individual.thread_count.clone().unwrap();
		let mut threadlock = thread.lock().unwrap();
		assert!(*threadlock == 0);
	
		*threadlock =1;	    
	}
	pool.lock().unwrap().execute(green,pool.clone());

	//Hacky
	while alloc::arc::strong_count(&pool) > 1
	{
		thread::sleep_ms(100);
	}


}



