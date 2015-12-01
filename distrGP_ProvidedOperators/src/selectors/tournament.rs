extern crate rand;
extern crate distrgp_generator;

use self::distrgp_generator::Graph;
use self::distrgp_generator::Selector;
use self::distrgp_generator::GlobalState;

use self::rand::distributions::{IndependentSample, Range};

#[derive(Debug,Clone)]
pub struct Tournament {
    tournament_size: u32,
}

impl Tournament
{

    pub fn new(size: u32) -> Tournament {
        Tournament { tournament_size: size }
    }

}

impl Selector for Tournament
{

    fn get_copy(&self) -> Box<Selector> {

        Box::new(self.clone()) as Box<Selector>

    }
    fn select(&self, pop: Vec<GlobalState>) -> Box<Fn() -> (Graph, Vec<u64>)> {

        let tournament_size = self.tournament_size;
        Box::new(move || {

                          assert!(pop.len() != 0, "Can't have a population size of 0");
                          let mut rng = rand::thread_rng();
                          let graph_count = Range::new(0, pop.len());



			//inefficient for small tournament sizes
                          let mut tournament_vector: Vec<GlobalState> = Vec::new();
                          for _ in 0..tournament_size {
                              tournament_vector.push(pop[graph_count.ind_sample(&mut rng)].clone());

                          }
                          tournament_vector.sort();
                          debug!("selected {0}, not selected {1}",tournament_vector[0].get_fitness(),tournament_vector[1].get_fitness());
                          tournament_vector[0].unique_graphvec_copy()

                      })



    }

}
