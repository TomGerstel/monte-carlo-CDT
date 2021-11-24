use structopt::StructOpt;

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

struct Timeslice {
    vertices: Vec<Vertex>,
}

struct Vertex {
    neighbours_up: Vec<VertexPos>,
    neighbours_down: Vec<VertexPos>,
}

struct VertexPos {
    time: usize,
    space: usize,
}
