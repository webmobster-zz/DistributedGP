extern crate csv;


use self::reader::ProblemDescription;
use super::servermessage::ServerMessage;

use super::enviroment;

use self::generator::Generator;
use self::generator::graph::Graph;
use self::generator::graph::visualize;

use std::sync::mpsc::sync_channel;
use std::sync::mpsc::channel;
use std::thread;

use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;

use std::ptr;




use std::io;
use std::fs::File;
use std::path::Path;

use std::fs;


pub mod generator;
mod reader;



pub fn init()
{

	//read problem description file

	let problem_description = reader::readfile();



	//server: Generate the initial population
	//popcount, operators,end_operators: Vec<uint>, fitness_function, point_mutate_probability, tree_mutate_probability, crossover_mutate_probability, selection_type)
	//this needs fixing badly	
	let mut generator = generator::Generator::init(
				problem_description.get_popcount(),
				problem_description.get_tree_size(),
				problem_description.get_operators(),
				problem_description.get_end_operators(),

				problem_description.get_operator_trait(),
				
				problem_description.get_parents(),
				problem_description.get_stats(),

				problem_description.get_point_mutate_probability(),
				problem_description.get_tree_mutate_probability(),
				problem_description.get_crossover_probability(),
				problem_description.get_flat_crossover_probability(),
				problem_description.get_point_remove_probability(),
				problem_description.get_clean_probability(),

				problem_description.get_repetitions(),

				problem_description.get_selection_type(),
				problem_description.get_life()
				);
	
	println!("created generator");
	generator.generate_graphs();
	println!("generated graphs");
	
	
	//println!("pop={}",population);

		
	//wait for client to connect OR launch local client threads
	let (receive, send) = wait_or_launch_clients(&problem_description);
	println!("launched clients");

	let numclients = send.len() as u32;


	//start clients
	start(&send);

	//send population to clients
	send_pop(&generator, &send);



	let(stdin_s, stdin_r) = channel();
	thread::spawn(move || {

				loop{
						let mut string = String::new();
						assert!(io::stdin().read_line(&mut string).is_ok());
						assert!(stdin_s.send(string).is_ok());
				}
			});


	let mut generation_number =1;
	let mut statsholder: Vec<Vec<(u32,u32,u32,u32)>> = Vec::new();
	
	loop
	{









			
			
			


	 

			//request scores from enviroments
			let updated_pop = get_scores(&receive,numclients);


			
			

			generator.set_graphs(updated_pop);

			let mut graphs = (*generator.graph_list).clone();
			graphs.sort();

			if generator.save_stats()
			{

				save_stats(&mut statsholder,&graphs);
			}

			
			println!("graph_fitness: {:1?},perfect: {:2?}, graph_length: {:3?} ",graphs[0].get_fitness(),graphs[0].get_perfect(), graphs[0].list.len());
			


			//Mutate and crossover new GAs
			generator.reproduce();
			println!("done with generation");
			

			//refactor this stuff into an I/O thread
			match stdin_r.try_recv()
			{
				Ok(string)=> { 
						if string == "x\n" {
									    if generator.save_stats()
									    {
											println!("saving stats");
											write_stats(&mut statsholder,generation_number);
									    }
									    write_stats(&mut statsholder,generation_number);
									    println!("exiting");
									    panic!("bad exit fix later");
								   }

					 	else if string == "sg\n" { println!("save graph");
									   write_graph(&graphs[0],generation_number);}

						else if string == "sc\n" { 
										    if generator.save_stats()
										    {
												println!("saving stats");
												write_stats(&mut statsholder,generation_number);
										    }
										    else
										    {
												println!("saving disabled in options");

										    }
									}
						else if string == "infpar\n" { 

										    if generator.get_parents()
										    {
												println!("saving parents");
												 write_parents(&graphs,generation_number);
										    }
										    else
										    {
												println!("saving disabled in options");

										    }
									    }


						else {println!("invalid message");}
					     }
				Err(_)=>()
			}

			//start clients
			start(&send);

			//send new population
			send_pop(&generator, &send);

			

			generation_number = generation_number+1;
	}

}


fn wait_or_launch_clients(problem_description: &ProblemDescription) -> (Receiver<ServerMessage>,Vec<SyncSender<ServerMessage>>)
{
	
	let (client_tx, server_listener): (SyncSender<ServerMessage>, Receiver<ServerMessage>) = sync_channel(30);
	
	let mut transmit_vector: Vec<SyncSender<ServerMessage>> = Vec::new();
	
	//number of threads fix?
	for _ in 0 .. problem_description.get_client_num()
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

fn send_pop(pop: &Generator, send: &Vec<SyncSender<ServerMessage>>)
{

	//create a list of problems
	let mut current= *pop.graph_list.clone();
	let mut vec_problems = Vec::new();

	for _ in 0..pop.get_repetitions()
	{
		let mut operator = pop.get_operator_trait();
		operator.init();

		vec_problems.push(operator);


	}	


	//split the population up equally amont the size of enviroments

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



			


	for i in 0..send.len()
	{


		//hack to clone the list of problems
		let mut op_trait_clone = Vec::new();
		for z in 0..vec_problems.len()
		{
			op_trait_clone.push(vec_problems[z].clone());

		}


		assert!(send[i].send(ServerMessage::OperatorTrait(op_trait_clone)).is_ok());
		assert!(send[i].send(ServerMessage::Repetitions(pop.get_repetitions())).is_ok());



		assert!(send[i].send(ServerMessage::PopVec( Box::new(index[i].clone()))).is_ok()); 
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


fn get_scores(receiver: &Receiver<ServerMessage>, num_clients: u32) -> Box<Vec<Graph>>
{
	

	let mut results = Box::new (Vec::new());

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


//All of this is IO
fn save_stats(stats_holder: &mut Vec<Vec<(u32,u32,u32,u32)>>, graphs: &Vec<Graph>)
{

		let mut new_vec: Vec<(u32,u32,u32,u32)> = Vec::new();

		for graph in graphs.iter()
		{
			let fitness = graph.get_fitness();
			let length = graph.list.len() as u32;
			let life =graph.life;
			let other =4;
			new_vec.push((fitness,length,life,other));
		}
		stats_holder.push(new_vec);

}
fn write_graph(graph: &Graph, generation_num: u32)
{
			let mut string_path = String::new();
			string_path.push_str("graph");
			string_path= string_path + &generation_num.to_string(); 
			string_path.push_str(".dot");
		    	let str_slice: &str = string_path.as_ref();
			let mut f = match File::create(&Path::new(str_slice))
					{
						Ok(x) => x,
						Err(_) => panic!("failed to create file")
		
					};
    			visualize::render_graph(&mut f,&graph);

}


fn write_parents(graphs: &Vec<Graph>, generation_num: u32)
{

			 for x in 0..graphs.len()
			{
				let mut string_path = String::new();
				string_path.push_str("parents");
				string_path= string_path + &generation_num.to_string(); 
				string_path.push_str("/graph");
				string_path= string_path + &x.to_string(); 

				let fs_string = string_path.clone();
				let fs_slice: &str = fs_string.as_ref();

				assert!(fs::create_dir_all(&Path::new(fs_slice)).is_ok());


				string_path.push_str("/graph.dot");
				let str_slice: &str = string_path.as_ref();
				let mut f = match File::create(&Path::new(str_slice))
				{
					Ok(x) => x,
					Err(_) => panic!("failed to create file")
		
				};
	    			visualize::render_graph(&mut f,&graphs[x]);

				let (parent1,parent2)= graphs[x].get_parents();


				if parent1.is_some()
				{
					let mut string_path = String::new();
					string_path.push_str("parents");
					string_path= string_path + &generation_num.to_string(); 
					string_path.push_str("/graph");
					string_path= string_path + &x.to_string(); 
					string_path.push_str("/parent1.dot");
					let str_slice: &str = string_path.as_ref();
				    	let mut f = match File::create(&Path::new(str_slice))
					{
						Ok(x) => x,
						Err(_) => panic!("failed to create file")
		
					};
		    			visualize::render_graph(&mut f,&*parent1.unwrap());
				}
			
				if parent2.is_some()
				{
					let mut string_path = String::new();
					string_path.push_str("parents");
					string_path= string_path + &generation_num.to_string(); 
					string_path.push_str("/graph");
					string_path= string_path + &x.to_string(); 
					string_path.push_str("/parent2.dot");
					let str_slice: &str = string_path.as_ref();
				    	let mut f = match File::create(&Path::new(str_slice))
					{
						Ok(x) => x,
						Err(_) => panic!("failed to create file")
		
					};
		    			visualize::render_graph(&mut f,&*parent2.unwrap());
				}
			}




}



fn write_stats(stats_holder: &mut  Vec<Vec<(u32,u32,u32,u32)>>, generation_num: u32)
{

	let mut string_path = String::new();
	string_path.push_str("output");
	string_path= string_path + &generation_num.to_string(); 
	string_path.push_str(".csv");
	let str_slice: &str = string_path.as_ref();
	let mut wtr = match csv::Writer::from_file(&Path::new(str_slice))
			{
				Ok(x) => x,
				Err(_) => panic!("failed to write stats")
		
			};

	for stats in stats_holder.clone().into_iter() {
    		let result = wtr.encode(stats);
   		 assert!(result.is_ok());
	}


}
