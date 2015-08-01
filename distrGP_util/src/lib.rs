#![feature(rustc_private)]
extern crate distrgp_evaluator;
extern crate distrgp_generator;

mod visualize;

use self::distrgp_generator::BiChannel;
use self::distrgp_generator::GlobalState;
use self::distrgp_evaluator::UtilMessage;

use std::io;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc::channel;


pub fn util_placeholder_runner(comms: BiChannel<UtilMessage>)
{
	let(stdin_s, stdin_r) = channel();
	thread::spawn(move || {

				loop{
						let mut string = String::new();
						assert!(io::stdin().read_line(&mut string).is_ok());
						assert!(stdin_s.send(string).is_ok());
				}
			});
	let mut generation_number=0;
	loop{

				match stdin_r.recv()
				{
					Ok(string)=> { 
						 	if string == "sg\n" { 		println!("save graph");
										   	assert!(comms.send(UtilMessage::RequestData).is_ok());
											let mut graphs;
											match comms.recv()
											{
												Ok(x) => match x
													{
														UtilMessage::Data(y)=>{graphs=y;},
														_=> panic!("invalid message")
													},
												_ => panic!("dropped util comms")
	
											}
											graphs.sort();
										   	write_graph(&graphs[0],generation_number);
											generation_number+=1;}

							else {println!("invalid message");}
						     }
					Err(_)=>panic!("dropped comms")
				}
	}
}



fn write_graph(state: &GlobalState, generation_num: u32)
{

 			println!("best={}",state.get_fitness());
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
			let (graph,_) = state.unique_graphvec_copy();
    			visualize::render_graph(&mut f,&graph);

}

/*
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
*/
