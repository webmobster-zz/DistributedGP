extern crate dot;

use std::io::Write;
use super::distrgp_generator::Graph;


type Nd<'a> = (usize);
type Ed<'a> = &'a (usize, usize,Option<bool>);
struct VizGraph { nodes: Vec<String>, edges: Vec<(usize,usize,Option<bool>)>, name: String }



pub fn render_graph<W:Write>(output: &mut W, graph: &Graph) {
    
	
	
    let nodes = graph.get_labeled_nodes();
    let edges = graph.get_edges();
    let viz_graph = VizGraph { nodes: nodes, edges: edges, name: "TheThing".to_string() };

    dot::render(&viz_graph, output).unwrap()
}


impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for VizGraph {
    fn graph_id(&'a self) -> dot::Id<'a>
    { 
		let str_slice : &str = self.name.as_ref();
		dot::Id::new(str_slice).unwrap()
    }
    
    fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n][..].into())
    }
    fn edge_label<'b>(&'b self, e: &Ed<'b>) -> dot::LabelText<'b> {
	//look at this
	let &(_,_,boolvar) =*e;
	if boolvar.is_some()
	{
        	dot::LabelText::LabelStr(boolvar.unwrap().to_string().into())
	}
	else
	{
		dot::LabelText::LabelStr("".into())
	}
    }
}

impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for VizGraph {

    fn nodes(&'a self) -> dot::Nodes<'a,Nd<'a>> {
       (0..self.nodes.len()).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> {
        self.edges.iter().collect()
    }
    fn source(&self, e: &Ed<'a>) -> Nd<'a> { let &&(s,_,_) = e; s }
    fn target(&self, e: &Ed<'a>) -> Nd<'a> { let &&(_,t,_) = e; t }
}



