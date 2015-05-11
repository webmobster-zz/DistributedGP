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


