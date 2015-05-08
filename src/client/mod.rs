


use self::envmessage::EnvMessage;
use self::envmessage::EnvMessage::{PopClient, EndPopClient, Startclient,RepetitionsEnv, OperatorTraitEnv};

use super::servermessage::ServerMessage;
use super::servermessage::ServerMessage::{Start,EndPop,PopVec, RepetitionsClient, OperatorTraitClient};

use super::server::generator::graph::Graph;


use super::server::generator::operator::OperatorTrait;


use std::sync::mpsc::sync_channel;

use std::thread;

use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;

use std::ptr;


mod enviroment;
mod envmessage;




// connect to server
pub fn init(server_send: SyncSender<ServerMessage>, server_receive: Receiver<ServerMessage>)
{
	// Initialise the Enviroment(s), set energy amounts, etc
	println!("client launched");
	let (env_rec, env_send, num_env) = spawn_env(Some(8));

	loop
	{

		// get start message from server
		
		match server_receive.recv()
		{

			Ok(msg) => {match msg
			{
				Start => {},
				_=> panic!("Invalid Message")
			};},
			Err(_)=> panic!("receive error")

		}



		//get fitness function
		let op_trait;

		match server_receive.recv()
		{

			Ok(msg) => {match msg
			{
				OperatorTraitClient(x) => {op_trait = x},
				_=> panic!("Invalid Message")
			};},
			Err(_)=> panic!("receive error")

		}



		let repetitions;

		match server_receive.recv()
		{

			Ok(msg) => {match msg
			{
				RepetitionsClient(x) => {repetitions = x},
				_=> panic!("Invalid Message")
			};},
			Err(_)=> panic!("receive error")

		}







		// receive population(s)
		let mut vec_pops;
		
		match server_receive.recv()
		{

			Ok(msg) => {match msg
			{
				PopVec(x) => vec_pops = x,
				_=> panic!("Invalid Message")
			};},
			Err(_)=> panic!("receive error")
		};	


		match server_receive.recv()
		{
			
			Ok(msg) => {match msg
			{
				EndPop => (),
				_=> panic!("Invalid Message")
			};},
			Err(_)=> panic!("receive error")
		}


		
		//start the next loop (allows changing of fitness function)
		start_env(&env_send,op_trait, repetitions);

		// Send population to enviroments

		send_pop(*vec_pops, &env_send);

		// get results from enviroments and push them up to server
		send_completed_to_server(&env_rec,&server_send,num_env);





		
	}
}

#[allow(deprecated)]
fn spawn_env(mut num_env: Option<usize>) -> (Receiver<EnvMessage>, Vec<SyncSender<EnvMessage>>,usize)
{

		// one thread per env
		match num_env
		{
 		  
		  None => {num_env = Some(1)}
		  _ =>{}
		}


		// information from enviroments to client
		let (env_tx, client_listener): (SyncSender<EnvMessage>, Receiver<EnvMessage>) = sync_channel(30);


		let mut transmit_vector: Vec<SyncSender<EnvMessage>> = Vec::new();
	
		for _ in 0 .. num_env.unwrap()
		{
			let(client_tx, enviroment_listener): (SyncSender<EnvMessage>, Receiver<EnvMessage>) = sync_channel(30);
			

			
			let local_tx = env_tx.clone();
			thread::spawn( move || { enviroment::init(local_tx,enviroment_listener)});
			transmit_vector.push(client_tx);

		}

		(client_listener,transmit_vector,num_env.unwrap())

}

fn send_pop(mut current: Vec<Graph>, send: &Vec<SyncSender<EnvMessage>>)
{

	let mut index: Vec<Vec<Graph>> = Vec::new();

	let length = current.len();

	let mut lastsplit =0;
	for i in 1 .. send.len()
	{

		let split = ((i as f32/send.len() as f32) * length as f32) as usize;
		let next = current.quick_hack_split( split -lastsplit );
		index.push(current);

		lastsplit=split;
		current = next;
	}
	index.push(current);
	for i in 0 .. send.len()
	{

			assert!(send[i].send(PopClient(Box::new(index[i].clone()))).is_ok());
			assert!(send[i].send(EndPopClient).is_ok());


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

fn send_completed_to_server(receiver: &Receiver<EnvMessage>, sender: &SyncSender<ServerMessage>, num_envs: usize) 
{



	for _ in 0 .. num_envs
	{

			match receiver.recv()
			{
				Ok(msg) => {match msg
				{
					PopClient(x) => 
					{
						assert!(sender.send(PopVec(x)).is_ok());
						
					}

					_=> panic!("Invalid Message")
				};},
				Err(_)=> panic!("receive error")

				
			};


	}
	assert!(sender.send(EndPop).is_ok());

}



fn start_env(send: &Vec<SyncSender<EnvMessage>>, op_trait: Vec<Box<OperatorTrait + Send>>, repetitions:u32 )
{

	for i in 0usize .. send.len()
	{

			assert!(send[i].send(Startclient).is_ok());

			let mut op_trait_clone = Vec::new();
			for z in 0..op_trait.len()
			{
				op_trait_clone.push(op_trait[z].clone());

			}
			
			assert!(send[i].send(OperatorTraitEnv(op_trait_clone)).is_ok());

			assert!(send[i].send(RepetitionsEnv(repetitions.clone())).is_ok());


	}

}


