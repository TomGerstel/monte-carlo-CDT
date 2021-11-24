use structopt::StructOpt;
use std::ops::Index;

/// A test to add two numbers
#[derive(StructOpt, Debug)]
#[structopt(name = "add_test")]
struct Opt {
    ///First number to add
    #[structopt(short, long)]
    alpha: usize,

    ///Second number to add
    #[structopt(short, long)]
    beta: usize,
}

fn main() {
    let opt = Opt::from_args();
    let a = opt.alpha;
    let b = opt.beta;
    println!("{:#?}", a + b);
}

struct Universe {
    slices: Vec<Timeslice>,
    order_four: Vec<Vertex>,
}

impl Index<VertexPos> for Universe {
    type Output = Vertex;

    fn index(&self, pos: VertexPos) -> &Self::Output {
        &self.slices[pos.time][pos.space]
    }
}

struct Timeslice {
    vertices: Vec<Vertex>,
}

impl Index<usize> for Timeslice {
    type Output = Vertex;

    fn index(&self, space: usize) -> &Self::Output {
        &self[space]
    }
}

struct Vertex {
    neighbours_up: Vec<VertexPos>,
    neighbours_down: Vec<VertexPos>,
}

struct VertexPos {
    time: usize,
    space: usize,
}