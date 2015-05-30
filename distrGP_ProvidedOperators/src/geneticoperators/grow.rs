pub fn grow_graph(operatorpointers: &Vec<Operator>, end_operators: &Vec<u32>, initial_size: u32, life: u32) -> Graph
	{

		if operatorpointers.len() ==0
		{
			panic!("no operators defined");

		}

		if end_operators.len() ==0
		{
			panic!("no end point operators defined");

		}


		let mut new_graph = Graph{list: Vec::with_capacity(initial_size as usize), perfect: None,fitness: None, life: life, parent1: None, parent2: None};


		let operator_count = Range::new(0, operatorpointers.len());

		//fast but bad
	   	let mut rng = rand::weak_rng();


		//intial node
		let operator = operatorpointers[operator_count.ind_sample(&mut rng)].clone();

		new_graph.add_to_end_node(operator);

		let loose_ends = Vec::new();

		//Grow behaviour comes from here
		loop
		{	



			while new_graph.list.len() < initial_size as usize
			{


				//no further succesors possible
				if loose_ends.len() == 0
				{
					break;
				}



				//deep behaviour
			

				//get loose end
				let working_index = loose_ends.pop().unwrap();
			

				let node = new_graph.list[working_index].clone();

				let Node(op,_,_,_) = node;

				let end = new_graph.list.len();
			
				if op.sucessors ==3
				{
					panic!("unimplemented feature");

				}
				
				else if op.sucessors ==2
				{
					//replace unlinked node with node with links
					new_graph.list[working_index] = Node(op,end as isize,(end +1) as isize,-1);

					//add new node with unfilled sucessors to the ends
					new_graph.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());
					new_graph.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());

				}
				else if op.sucessors ==1
				{

					new_graph.list[working_index] = Node(op,end as isize,-1,-1);

					//add new node with unfilled sucessors to the ends
					new_graph.add_to_end_node(operatorpointers[operator_count.ind_sample(&mut rng)].clone());

				}
				else if op.sucessors ==0
				{
					
					//do nothing, as the Node should have unconnected sucessors already
					//hopefully this will get compiled away, usefull for code readability sakes


				}



			}


			//CLEAR UP DANGLING NODES
			

			//no further succesors possible
			if loose_ends.len() == 0
			{
				break;
			}


			let end_operator_count = Range::new(0, end_operators.len());
			
			//get a random index from the end operator list, which is used to get an operator from the operator list
			//SAME OPERATOR EVERY TIME FIX THIS!
			let operator = operatorpointers[end_operators[end_operator_count.ind_sample(&mut rng)] as usize].clone();

			let working_index = loose_ends.pop().unwrap();
			
			let node = new_graph.list[working_index].clone();

			let Node(op,_,_,_) = node;

			let end = new_graph.list.len();


			if op.sucessors ==3
			{
				panic!("unimplemented feature");

			}
			
			else if op.sucessors ==2
			{
				//replace unlinked node with node with links
				new_graph.list[working_index] = Node(op,end as isize,(end +1) as isize,-1);

				//add new node with unfilled sucessors to the ends
				new_graph.add_to_end_node(operator.clone());
				new_graph.add_to_end_node(operator.clone());

			}
			else if op.sucessors ==1
			{
				//replace unlinked node with node with link
				new_graph.list[working_index] = Node(op,end as isize,-1,-1);

				//add new node with unfilled sucessors to the ends
				new_graph.add_to_end_node(operator);

			}
			else if op.sucessors ==0
			{

				//do nothing, as the Node should have unconnected sucessors already
				//hopefully this will get compiled away, usefull for code readability sakes



			}

		}
		new_graph

	}
