pub fn placeholder_io()
{


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
