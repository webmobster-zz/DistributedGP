#[derive(Clone)]
pub struct GlobalState
{
	pub vec: Vec<u64>,
	pub vec_pointer: usize
}

pub struct LocalState
{
	pub node: Option<usize>,
	pub array: [u64;1000],
	pub local_pointer: usize

	
}

impl LocalState
{
	pub fn new() -> LocalState
	{

		LocalState{node: Some(0), array: [0;1000],local_pointer: 0}

	}

}

