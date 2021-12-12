use structopt::StructOpt;
mod universe;

/// A Markov Chain Monte Carlo simulation of 2-dimensional Causal Dynamical Triangulations.
#[derive(StructOpt, Debug)]
#[structopt(name = "monte_carlo_CDT")]
struct Opt {
    /// Number of timeslices
    #[structopt(short = "t", long)]
    timespan: usize,

    /// Number of triangles
    #[structopt(short = "n", long)]
    triangle_count: usize,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the equilibration phase
    #[structopt(short = "b", long)] // TODO: fill in default value when we know optimal value
    move_ratio_eq: f32,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the measurement phase
    #[structopt(short = "r", long)] // TODO: fill in default value when we know optimal value
    move_ratio_meas: f32,

    /// Length of equilibration phase in sweeps
    #[structopt(short = "e", long)]
    eq_sweeps: usize,

    /// Length of measurement phase in sweeps
    #[structopt(short = "m", long)]
    meas_sweeps: usize,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "monte_carlo_CDT")]
struct OptSimple {
    /// Number of timeslices
    #[structopt(short = "t", long)]
    timespan: usize,

    /// Number of triangles
    #[structopt(short = "n", long)]
    triangle_count: usize,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the equilibration phase
    #[structopt(short = "r", long)] // TODO: fill in default value when we know optimal value
    move_ratio: f32,

    /// Amount of Markov Chain Monte Carlo steps
    #[structopt(short = "s", long)]
    steps: usize,
}

// example command (on Windows):
// target\release\monte-carlo-cdt.exe -t 20 -n 800 -b 0.7 -r 0.5 -e 10 -m 100
fn main() {
    //simple_measurement();
    let mut universe = universe::Universe::new(2, 8);
    for _ in 0..100000 {
        universe.mcmc_step(0.5);
    }
    dbg!(universe.lengths(5));
}

fn simple_measurement() {
    // A simpler CLI for preliminary data-analysis which simply performs the model for a given amount of steps

    // set parameters
    let opt = OptSimple::from_args();
    let timespan = opt.timespan;
    let triangle_count = opt.triangle_count;
    let move_ratio = opt.move_ratio;
    let steps = opt.steps;

    // check move ratio parameters
    assert!(
        (0.0..=1.0).contains(&move_ratio),
        "given move ratio ({}) is outside valid range [0.0, 1.0]",
        move_ratio
    );

    // measurements
    let mut universe = universe::Universe::new(timespan, triangle_count);
    for _ in 0..steps {
        universe.mcmc_step(move_ratio);
        for length in universe.lengths(0) {
            print!("{} ", length);
        }
        println!("");
    }
}

fn full_measurement() {
    // set parameters
    let opt = Opt::from_args();
    let timespan = opt.timespan;
    let triangle_count = opt.triangle_count;
    let move_ratio_eq = opt.move_ratio_eq;
    let move_ratio_meas = opt.move_ratio_meas;
    let eq_sweeps = opt.eq_sweeps;
    let meas_sweeps = opt.meas_sweeps;

    // check move ratio parameters
    assert!(
        (0.0..=1.0).contains(&move_ratio_eq),
        "given move ratio ({}) is outside valid range [0.0, 1.0]",
        move_ratio_eq
    );
    assert!(
        (0.0..=1.0).contains(&move_ratio_meas),
        "given move ratio ({}) is outside valid range [0.0, 1.0]",
        move_ratio_meas
    );

    // define sweeps
    let sweep = triangle_count / 2;

    // equilibration phase
    let mut universe = universe::Universe::new(timespan, triangle_count);
    for _ in 0..(eq_sweeps * sweep) {
        universe.mcmc_step(move_ratio_eq);
    }

    // measurement phase
    for _ in 0..(meas_sweeps * sweep) {
        universe.mcmc_step(move_ratio_meas);
        // TODO: do measurments here
    }
}
