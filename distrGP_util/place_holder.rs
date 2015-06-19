


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
