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

    let mut universe = universe::Universe::new(200, 80_000);
    for _ in 0..500_000 {
        universe.mcmc_step(0.5);
        // assert!(universe.check_order_four_list(), "The order four list contains none order four things");
        // assert!(universe.check_all_order_four(), "The order four list is no longer correct");
    }
    // println!("{:?}", universe.lengths(0));
}
