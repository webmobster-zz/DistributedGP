extern crate distrGP_Generator;





use std::sync::mpsc::sync_channel;
use std::thread;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::ptr;



use super::enviroment;

use self::distrGP_Generator::Generator;
use self::distrGP_Generator::IndividualComm;
use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::GlobalState;

pub enum FitnessMessage
{
	Ready,
	PopVec(Vec<IndividualComm>),
	Finish

}

pub enum ServerMessage
{
	Start,
	PopVec(Vec<GlobalState>),
	OperatorMap(OperatorMap),
	Repetitions(u32),
	EndPop

}

pub fn init(mut generator: Generator, numclients: u32, sender: Sender<FitnessMessage>, receiver: Receiver<FitnessMessage>)
{


	

	assert!(numclients > 0, "Need to set more than 1 evaluator threads");





	generator.generate_graphs();


	info!("generated graphs");
	

		
	//wait for client to connect OR launch local client threads
	let (receive, send) = launch_enviroments(numclients);
	println!("launched clients");

	assert!( numclients == send.len() as u32, "Launching enviroments failed");

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
		
		//start clients
		start(&send);

		//send population to clients
		send_pop(&generator, &send);


		//request scores from enviroments
		let updated_pop = get_scores(&receive,numclients);


			
			

		generator.set_graphs(updated_pop);



		generator.reproduce();
		sender.send(FitnessMessage::Finish);

	}

			




}


fn get_scores(receiver: &Receiver<ServerMessage>, num_clients: u32) -> Vec<GlobalState>
{
	

	let mut results = Vec::new();

	let mut done_clients = 0;
	while done_clients<num_clients
	{	

		match receiver.recv()
		{
			Ok(msg) => {match msg
			{
				ServerMessage::PopVec(x) => {
							for i in 0..x.len()
							{
								results.push(x[i].clone());
							}

						},
				ServerMessage::EndPop => done_clients +=1,
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
		}


	}
	results


}


fn launch_enviroments(enviroment_number: u32) -> (Receiver<ServerMessage>,Vec<SyncSender<ServerMessage>>)
{
	
	let (client_tx, server_listener): (SyncSender<ServerMessage>, Receiver<ServerMessage>) = sync_channel(30);
	
	let mut transmit_vector: Vec<SyncSender<ServerMessage>> = Vec::new();
	
	//number of threads fix?
	for _ in 0 .. enviroment_number
	{
		let (server_tx, client_listener): (SyncSender<ServerMessage>, Receiver<ServerMessage>) = sync_channel(20);
			

			
		let local_tx = client_tx.clone();
		thread::spawn( move || { enviroment::init(local_tx,client_listener)});

		transmit_vector.push(server_tx);

	}

	(server_listener,transmit_vector)

}

fn start(send: &Vec<SyncSender<ServerMessage>>)
{
	for i in 0usize .. send.len()
	{

		assert!(send[i].send(ServerMessage::Start).is_ok());
		

	}

}


//maybe get rid of the &generator
fn send_pop(generator: &Generator, send: &Vec<SyncSender<ServerMessage>>)
{

	//create a list of problems
	let mut current= generator.get_graph_list();

	let mut map = generator.get_operator_map();




	//split the population up equally amont the size of enviroments

	let mut index: Vec<Vec<GlobalState>> = Vec::new();
	let length = current.len();
	
	let mut lastsplit =0;
	for i in 1 .. send.len()
	{

		let split = ((i as f32/send.len() as f32) * length as f32) as usize;
		let next: Vec<GlobalState> = current.quick_hack_split( split -lastsplit );
		index.push(current);

		lastsplit=split;
		current = next;
	}
	index.push(current);



			


	for i in 0..send.len()
	{



		assert!(send[i].send(ServerMessage::OperatorMap(map.clone())).is_ok());



		assert!(send[i].send(ServerMessage::PopVec( index[i].clone())).is_ok()); 
		assert!(send[i].send(ServerMessage::EndPop).is_ok());


	}
	

}


trait Hack 
{
	fn quick_hack_split(&mut self, at: usize) -> Self;
}

impl<T> Hack for Vec<T> {

	fn quick_hack_split(&mut self, at: usize) -> Self {
		assert!(at <= self.len(), "`at` out of bounds");

		let other_len = self.len() - at;
		let mut other = Vec::with_capacity(other_len);

		// Unsafely `set_len` and copy items to `other`.
		unsafe {
		    self.set_len(at);
		    other.set_len(other_len);

		    ptr::copy_nonoverlapping(
		        self.as_ptr().offset(at as isize),
		        other.as_mut_ptr(),
		        other.len());
		}
		other
	}
}



