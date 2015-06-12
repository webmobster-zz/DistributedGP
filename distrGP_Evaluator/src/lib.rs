#![feature(scoped)] 
#![feature(alloc)] 
#[macro_use]
extern crate log;
extern crate alloc;
extern crate distrGP_Generator;
mod pool;

use std::sync::mpsc::Sender;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::sync_channel;
use std::ptr;

use self::distrGP_Generator::Graph;
use self::distrGP_Generator::Operator;
use self::distrGP_Generator::Generator;
use self::distrGP_Generator::IndividualComm;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::GlobalState;
use self::distrGP_Generator::LocalState;
use self::distrGP_Generator::StateIO;


use self::pool::ThreadPool;
use self::pool::GreenThreadData;

pub enum FitnessMessage
{
	Ready,
	PopVec(Vec<IndividualComm>),
	Finish

}

pub fn init(mut generator: Generator, numthreads: u32, sender: Sender<FitnessMessage>, receiver: Receiver<FitnessMessage>)
{


	

	assert!(numthreads > 0, "Need to set more than 1 evaluator threads");





	generator.generate_graphs();


	info!("generated graphs");
	

		
	let pool= Arc::new(Mutex::new(ThreadPool::new(12)));



	loop
	{

		let comms= generator.initialize_graphs();
		sender.send(FitnessMessage::PopVec(comms));

		info!("waiting for fitness to be ready clients");
		match receiver.recv()
		{
			Ok(x) => {
				 	match x
					{
						FitnessMessage::Ready => (),
						_ => panic!("Invalid Message")
					}
				},
			_ => panic!("Dropped receiver")
		}

		

		// run all populations and send fitnesses 
		//try and get rid of the map clones
		debug!("Fix Me: multiple uneeded clones of operator map, as the borrow check cant confirm all the jobs using the map will be finished before the next loop");
		let opmap =generator.get_operator_map();
		iterate_over_entity(generator.get_graph_list_mutref(),opmap.clone(),pool.clone());

			

		//generator.set_graphs(updated_pop);



		generator.reproduce();
		sender.send(FitnessMessage::Finish);

	}

			




}


fn iterate_over_entity(pop: &mut Vec<GlobalState>, map: OperatorMap, pool: Arc<Mutex<ThreadPool>>)
{


	for i in 0 .. pop.len()
	{
		let working_graph = (&mut **pop).get_mut(i).unwrap();

		let initial_local_state = LocalState::new();


		let green= GreenThreadData::new(working_graph.clone(),initial_local_state,map.clone());

		{
			let thread = working_graph.thread_count.clone().unwrap();
			let mut threadlock = thread.lock().unwrap();
			assert!(*threadlock == 0);
	
			*threadlock =1;	    
		}
		pool.lock().unwrap().execute(green,pool.clone());
	}
	//Hacky
	while alloc::arc::strong_count(&pool) > 2
	{
		thread::sleep_ms(100);
	}


}	




