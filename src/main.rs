use structopt::StructOpt;
mod universe;
use serde_json::json;
use std::time::SystemTime;

/// A Markov Chain Monte Carlo simulation of 2-dimensional Causal Dynamical Triangulations.
#[derive(StructOpt, Debug)]
#[structopt(name = "monte_carlo_CDT")]
struct Opt {
    /// Number of timeslices
    #[structopt(short = "t", long)]
    timespan: usize,

    /// Number of measurements to be performed
    #[structopt(short = "n", long)]
    n_meas: usize,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the equilibration phase
    #[structopt(short = "b", long, default_value = "0.5")]
    // TODO: adjust default value when we know optimal value
    move_ratio_eq: f32,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the measurement phase
    #[structopt(short = "r", long, default_value = "0.5")]
    // TODO: adjust default value when we know optimal value
    move_ratio_meas: f32,

    /// Length of equilibration phase in sweeps
    #[structopt(short = "e", long, default_value = "0")]
    eq_sweeps: usize,

    /// Number of sweeps inbetween measurements
    #[structopt(short = "p", long, default_value = "0")]
    pause: usize,
}

// example command (on Windows):
// target\release\monte-carlo-cdt.exe -t 20 -n 100
fn main() {
    let _ = measurement();
}

fn measurement() -> std::io::Result<()> {
    // set parameters
    let opt = Opt::from_args();
    let timespan = opt.timespan;
    let n_meas = opt.n_meas;
    let move_ratio_eq = opt.move_ratio_eq;
    let move_ratio_meas = opt.move_ratio_meas;
    let eq_sweeps = opt.eq_sweeps;
    let pause = opt.pause;

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

    // define sweeps and Markov Chain time
    let sweep = timespan * timespan;
    let mut t_mc = 0;

    // equilibration phase
    let mut universe = universe::Universe::new(timespan);
    for _ in 0..(eq_sweeps * sweep) {
        universe.mcmc_step(move_ratio_eq);
    }

    // create data structures to store generated data
    #[derive(serde::Serialize)]
    struct Datum {
        t_mc: usize,
        lengths: Vec<usize>,
    }
    let mut data = Vec::with_capacity(n_meas);

    // measurement phase
    for _ in 0..n_meas {
        for _ in 0..(pause * sweep + 1) {
            universe.mcmc_step(move_ratio_meas);
            t_mc += 1;
        }
        // perform measurements
        let origin = fastrand::usize(0..(2 * timespan * timespan));
        let lengths = universe.lengths(origin);
        let datum = Datum { t_mc, lengths };
        data.push(datum);
    }

    // put everything in json format
    let measurement = json!({
        "parameters": {
            "timespan": timespan,
            "n_meas": n_meas,
            "move_ratio_eq": move_ratio_eq,
            "move_ratio_meas": move_ratio_meas,
            "eq_sweeps": eq_sweeps,
            "pause": pause,
        },
        "data": data
    });

    // write to file
    let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    let path = format!("data/{}.json", time);
    let content = measurement.to_string();
    std::fs::write(path, content)?;
    Ok(())
}
