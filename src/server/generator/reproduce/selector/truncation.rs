use super::Graph;
pub fn truncate(mut pop: &mut Box<Vec<Graph>>, amount_kept: uint  )
{
	(&mut **pop).sort();
	if pop[0].get_fitness() == 0
	{
		println!("Done, pop list: {}, ", pop[0]);
		panic!("quitting ungracefully XD");
	}

	pop.truncate(amount_kept);
}
