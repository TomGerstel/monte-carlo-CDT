use structopt::StructOpt;
mod universe;
pub use universe::*;

/// A test to add two numbers
#[derive(StructOpt, Debug)]
#[structopt(name = "add_test")]
struct Opt {
    /// First number to add
    #[structopt(short, long)]
    alpha: usize,

    /// Second number to add
    #[structopt(short, long)]
    beta: usize,
}

fn main() {
    //let opt = Opt::from_args();
    //let a = opt.alpha;
    //let b = opt.beta;
    //println!("{:#?}", a + b);

    let mut universe = universe::Universe::new(20, 800);
    for _ in 0..100000 {
        universe.mcmc_step(0.5);
    }
    dbg!(universe.lengths(5));
}
