extern crate rand;
extern crate distrgp_generator;

use self::distrgp_generator::Graph;
use self::distrgp_generator::Node;
use self::distrgp_generator::OperatorMap;
use self::distrgp_generator::GeneticOperator;
use self::distrgp_generator::RandomKey;

use self::rand::distributions::{IndependentSample, Range};
use self::rand::Rng;
use std::collections::VecDeque;
use std::collections::HashMap;


#[derive(Debug,Clone)]
pub struct PointMutate {
    probability: f32,
}

impl PointMutate
{

    pub fn new(probability: f32) -> PointMutate {
        PointMutate { probability: probability }
    }

}

impl GeneticOperator for PointMutate
{


    fn get_copy(&self) -> Box<GeneticOperator> {

        Box::new(self.clone()) as Box<GeneticOperator>

    }

    fn get_probability(&self) -> f32 {

        self.probability

    }

    fn operate(&self,
               map: &mut OperatorMap,
               selector_closure: &Box<Fn() -> (Graph, Vec<u64>)>)
               -> Vec<(Graph, Vec<u64>)> {

        let (mut working_graph,vec) = selector_closure();


		//println!("before tree mutation: {:?}",working_graph);
		//fast but bad
        let mut rng = rand::weak_rng();

		//fix bad OO practices, getters and setters etc
        let graph_length = Range::new(0, working_graph.get_size());

        let working_index = graph_length.ind_sample(&mut rng);

        let mut working_node = working_graph.get_node(working_index);

		//assume sucessor count is correct
        let Node(_,mut suc1,mut suc2) = working_node.clone();


        let new_operator = map.random_key(&mut rng);



		// has to be a nicer way

		//matches successors, either prunes or generates a random sucessor if non exist.

        if map[&new_operator].get_sucessors() == 0 {

            working_node = Node(new_operator.clone(), None, None)

        }

        if map[&new_operator].get_sucessors() == 1 {
            if suc1 == None {
                suc1 = Some(graph_length.ind_sample(&mut rng));

            }

            working_node = Node(new_operator.clone(), suc1, None)

        }
        if map[&new_operator].get_sucessors() == 2 {

            if suc1 == None {
                suc1 = Some(graph_length.ind_sample(&mut rng));

            }

            if suc2 == None {
                suc2 = Some(graph_length.ind_sample(&mut rng));

            }

            working_node = Node(new_operator.clone(), suc1, suc2)

        }



        working_graph.set_node(working_index,working_node);


		//println!("point tree mutation: {:?}",working_graph);
        vec!((working_graph,vec))
    }
}


#[derive(Debug,Clone)]
pub struct Rewire {
    probability: f32,
}

impl Rewire
{

    pub fn new(probability: f32) -> Rewire {
        Rewire { probability: probability }
    }

}

impl GeneticOperator for Rewire
{


    fn get_copy(&self) -> Box<GeneticOperator> {

        Box::new(self.clone()) as Box<GeneticOperator>

    }

    fn get_probability(&self) -> f32 {

        self.probability

    }

    fn operate(&self,
               map: &mut OperatorMap,
               selector_closure: &Box<Fn() -> (Graph, Vec<u64>)>)
               -> Vec<(Graph, Vec<u64>)> {

        let (mut working_graph,vec) = selector_closure();


		//println!("before tree mutation: {:?}",working_graph);
		//fast but bad
        let mut rng = rand::weak_rng();

		//fix bad OO practices, getters and setters etc
        let graph_length = Range::new(0, working_graph.get_size());

        let working_index = graph_length.ind_sample(&mut rng);

        let mut working_node = working_graph.get_node(working_index);

		//assume sucessor count is correct
        let Node(op,_,_) = working_node.clone();



        if map[&op].get_sucessors() == 0 {

            working_node = Node(op, None, None)

        }

        if map[&op].get_sucessors() == 1 {

            let suc1 = Some(graph_length.ind_sample(&mut rng));
            working_node = Node(op, suc1, None)

        }
        if map[&op].get_sucessors() == 2 {

            let suc1 = Some(graph_length.ind_sample(&mut rng));
            let suc2 = Some(graph_length.ind_sample(&mut rng));
            working_node = Node(op, suc1, suc2)

        }



        working_graph.set_node(working_index,working_node);


		//println!("point tree mutation: {:?}",working_graph);
        vec!((working_graph,vec))
    }
}
#[derive(Debug,Clone)]
pub struct InsertNode {
    probability: f32,
}

impl InsertNode
{

    pub fn new(probability: f32) -> InsertNode {
        InsertNode { probability: probability }
    }

}

impl GeneticOperator for InsertNode
{


    fn get_copy(&self) -> Box<GeneticOperator> {

        Box::new(self.clone()) as Box<GeneticOperator>

    }

    fn get_probability(&self) -> f32 {

        self.probability

    }

    fn operate(&self,
               map: &mut OperatorMap,
               selector_closure: &Box<Fn() -> (Graph, Vec<u64>)>)
               -> Vec<(Graph, Vec<u64>)> {

        let (mut working_graph,vec) = selector_closure();


        let mut rng = rand::weak_rng();

        let graph_length = Range::new(0, working_graph.get_size());

        let working_index = graph_length.ind_sample(&mut rng);

        let mut working_node = working_graph.get_node(working_index);




        let old_node = working_node.clone();


        let new_operator = map.random_key(&mut rng);


		//new node will be in end position
        let new_position = working_graph.get_size();



        if map[&new_operator].get_sucessors() == 0 {

            working_node = Node(new_operator.clone(), None, None)

        }

        if map[&new_operator].get_sucessors() == 1 {

            working_node = Node(new_operator.clone(), Some(new_position), None)

        }
        if map[&new_operator].get_sucessors() == 2 {


            let suc = Some(graph_length.ind_sample(&mut rng));
			//alternate if the true or false is the new destination
            if rng.gen::<bool>() {
                working_node = Node(new_operator.clone(), Some(new_position), suc)
            } else {
                working_node = Node(new_operator.clone(), suc, Some(new_position))
            }

        }
        working_graph.add_to_end(old_node);


        working_graph.set_node(working_index,working_node);


		//println!("point tree mutation: {:?}",working_graph);
        vec!((working_graph,vec))
    }
}

#[derive(Debug,Clone)]
pub struct Clean {
    probability: f32,
}

impl Clean
{

    pub fn new(probability: f32) -> Clean {
        Clean { probability: probability }
    }

}

impl GeneticOperator for Clean
{


    fn get_copy(&self) -> Box<GeneticOperator> {

        Box::new(self.clone()) as Box<GeneticOperator>

    }

    fn get_probability(&self) -> f32 {

        self.probability

    }

    fn operate(&self,
               map: &mut OperatorMap,
               selector_closure: &Box<Fn() -> (Graph, Vec<u64>)>)
               -> Vec<(Graph, Vec<u64>)> {

        let (mut working_graph,vec) = selector_closure();



        let mut unfinished_nodes = VecDeque::new();

        let mut index_map: HashMap<usize, usize> = HashMap::new();


        let start_get: usize = 0;
        let start_replace: usize = 0;

		//possibly more efficient way to do this
        index_map.insert(start_get,start_replace);

        unfinished_nodes.push_back((start_get,start_replace));


		//probably best not to write to the same data structure we are reading
        let old = working_graph.clone();



		//keeps track of allocated positions

        working_graph = Graph::empty_graph();


		//increment by one cause position 0 is needs to be allocated to the first node
        let mut last_used_position = working_graph.get_size() + 1;

        while !unfinished_nodes.is_empty() {



            let (current_get_index,current_put_index) = match unfinished_nodes.pop_front() {
                Some(x) => x,
                None => panic!("should never happen"),
            };



			//does the current node exist, or is it getting put on the end
            let increase_graph_size;


			//should only occur in loops?
            if current_put_index < working_graph.get_size() {
				//panic!("loop");
                increase_graph_size = false;
            } else if working_graph.get_size() == current_put_index {

                increase_graph_size = true;
				//last_used_position = last_used_position +1;
            } else {
                panic!("error in logic somewhere");
            }





			//make sure wrapping is accounted for
			//removed wrapping 4 testing purposes  % old.list.len()
            let mut current_node = old.get_node(current_get_index  % old.get_size());




            let Node(op, suc1,suc2,) = current_node.clone();


			//THE END LOGIC should be fixed
            if map[&op].get_sucessors() == 2 {
                assert!(suc1.is_some() && suc2.is_some(),"Invalid Node");


                let suc1 = suc1.unwrap();
                let suc2 = suc2.unwrap();

					//problem seems to be here suc1s sucessorts will pop before suc2 does
                if !index_map.contains_key(&suc1) && !index_map.contains_key(&suc2) {

                    current_node = Node(op,
                                        Some(last_used_position),
                                        Some((last_used_position + 1)));



                    index_map.insert(suc1,last_used_position);
                    unfinished_nodes.push_back((suc1,last_used_position));
                    last_used_position = last_used_position + 1;


                    index_map.insert(suc2,last_used_position);
                    unfinished_nodes.push_back((suc2,last_used_position));
                    last_used_position = last_used_position + 1;

                } else if index_map.contains_key(&suc1) && index_map.contains_key(&suc2) {
                    current_node = Node(op,
                                        Some(*index_map.get(&suc1).unwrap()),
                                        Some(*index_map.get(&suc2).unwrap()));
                } else if index_map.contains_key(&suc1) {
                    current_node = Node(op,
                                        Some(*index_map.get(&suc1).unwrap()),
                                        Some(last_used_position));
                    index_map.insert(suc2,last_used_position);
                    unfinished_nodes.push_back((suc2,last_used_position));
                    last_used_position = last_used_position + 1;
                } else {
                    current_node = Node(op,
                                        Some(last_used_position),
                                        Some(*index_map.get(&suc2).unwrap()));
                    index_map.insert(suc1,last_used_position);
                    unfinished_nodes.push_back((suc1,last_used_position));
                    last_used_position = last_used_position + 1;

                }



            } else if map[&op].get_sucessors() == 1 {
                assert!(suc1.is_some() && suc2.is_none(),"Invalid Node");

                let suc1 = suc1.unwrap();
                if !index_map.contains_key(&suc1) {
                    current_node = Node(op, Some(last_used_position), None);
                    index_map.insert(suc1,last_used_position);
                    unfinished_nodes.push_back((suc1,last_used_position));
                    last_used_position = last_used_position + 1;
                } else {
                    current_node = Node(op, Some(*index_map.get(&suc1).unwrap()), None);
                }



            } else if map[&op].get_sucessors() == 0 {
                assert!(suc1.is_none() && suc2.is_none(),"Invalid Node");




            }


			//replace or grow the list
            if !increase_graph_size {
                working_graph.set_node(current_put_index,current_node);
            } else {

                working_graph.add_to_end(current_node);

            }


        }
        vec!((working_graph,vec))

    }




}


/*
#[derive(Debug,Clone)]
pub struct TreeMutate
{

	probability:f32,
	depth: u64
}

impl TreeMutate
{

	pub fn new(probability: f32) -> TreeMutate
	{
		Clean{probability: probability}
	}

}

impl GeneticOperator for TreeMutate
{


	fn get_copy(&self) ->  Box<GeneticOperator>
	{

		Box::new(self.clone()) as Box<GeneticOperator>

	}

	fn get_probability(&self) ->  f32
	{

		self.probability

	}

	fn operate(&self,  map: &mut OperatorMap,selector_closure: &Box<Fn() -> (Graph,Vec<u64>)>) -> Vec<(Graph,Vec<u64>)>
	{

		let mut working_graph= selector.select();

		let mut working_graph= match selection_type
		{
					Tournament(k) => selector::tournament_selection(generator,k),
					_ => panic!("unimplemented code")
		};


		//println!("before tree mutation: {:?}",working_graph);


		//fast but bad
		let mut rng = rand::weak_rng();


		let graph_length = Range::new(0, working_graph.list.len());

		let working_index = graph_length.ind_sample(&mut rng);


		//hard coded subtree size
		working_graph.grow_new_subtree(&generator.operatorpointers,&generator.end_operators,working_index,5);


		//working_graph.clean();

		//println!("after tree mutation: {:?}",working_graph);
		working_graph
	}

}
*/
/*
pub fn point_remove(generator: & mut Generator) -> Graph
{

	let mut working_graph= selector.select();



	//println!("before tree mutation: {:?}",working_graph);
	//fast but bad
	let mut rng = rand::weak_rng();

	//fix bad OO practices, getters and setters etc
	let graph_length = Range::new(0, working_graph.list.len());

	let working_index = graph_length.ind_sample(&mut rng);

	if working_graph.list.len() > 1
	{
		working_graph.remove_node(working_index);
	}

	working_graph

}



*/
