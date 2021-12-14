use structopt::StructOpt;
mod universe;
use serde_json::json;
use std::time::SystemTime;

/// A Markov Chain Monte Carlo simulation of 2-dimensional Causal Dynamical Triangulations.
#[derive(StructOpt, Debug, serde::Serialize)]
#[structopt(name = "monte_carlo_CDT")]
struct Opt {
    /// Number of timeslices
    #[structopt(short = "t", long)]
    timespan: usize,

    /// Average number of links per timeslice
    #[structopt(short = "l", long)]
    length: usize,

    /// Number of Markov Chain timesteps to save
    #[structopt(short = "n", long)]
    n_save: usize,

    /// Probability of performing a shard move for a single Markov chain step
    /// outside the equilibration phase
    #[structopt(short = "r", long)]
    // TODO: adjust default value when we know optimal value
    move_ratio: f32,

    /// Option to choose between doing a measurement or test run
    #[structopt(short = "m", long)]
    is_measurement: bool,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the equilibration phase
    #[structopt(short = "e", long, default_value = "0.5")]
    // TODO: adjust default value when we know optimal value
    move_ratio_eq: f32,

    /// Length of equilibration phase in sweeps (bake-in)
    #[structopt(short = "b", long, default_value = "0")]
    eq_sweeps: usize,

    /// Number of sweeps inbetween measurements (pause)
    #[structopt(short = "p", long, default_value = "0")]
    pause: usize,
}

// example commands (on Windows):
// cargo build --release
// target\release\monte-carlo-cdt.exe -t 10 -l 100 -n 100 -r 0.5
fn main() {
    let _ = measurement();
}

fn measurement() -> std::io::Result<()> {
    // set parameters
    let opt = Opt::from_args();
    let timespan = opt.timespan;
    let length = opt.length;
    let n_save = opt.n_save;
    let move_ratio = opt.move_ratio;
    let is_measurement = opt.is_measurement;

    // check move ratio parameter
    assert!(
        (0.0..=1.0).contains(&move_ratio),
        "given move ratio ({}) is outside valid range [0.0, 1.0]",
        move_ratio
    );

    // define data structures to store data and parameters
    #[derive(serde::Serialize)]
    enum Datum {
        LengthProfile(Vec<usize>),
        LengthVar(f32),
    }

    // big bang
    let mut universe = universe::Universe::new(timespan, length);
    let sweep = 2 * timespan * length;

    // do equilibration phase if required
    if is_measurement {
        let move_ratio_eq = opt.move_ratio_eq;
        let eq_sweeps = opt.eq_sweeps;

        // check equilibration move ratio parameter
        assert!(
            (0.0..=1.0).contains(&move_ratio_eq),
            "given move ratio ({}) is outside valid range [0.0, 1.0]",
            move_ratio_eq
        );

        // equilibration phase
        for _ in 0..(eq_sweeps * sweep) {
            universe.mcmc_step(move_ratio_eq);
        }
    }

    // determine the number of timesteps between measurements
    let pause = match is_measurement {
        true => opt.pause * sweep,
        false => 1,
    };

    // measurement phase
    let mut data = Vec::with_capacity(n_save);
    for _ in 0..n_save {
        for _ in 0..pause {
            universe.mcmc_step(move_ratio);
        }
        let datum = match is_measurement {
            true => {
                let origin = fastrand::usize(0..sweep);
                Datum::LengthProfile(universe.length_profile(origin))
            }
            false => Datum::LengthVar(universe.length_var()),
        };
        data.push(datum);
    }

    // put everything in json format
    let measurement = json!({
        "parameters": opt,
        "data": data
    });

    // get the current time to put into the filename
    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    // create the filename prefix
    let data_type = match is_measurement {
        true => "meas",
        false => "test",
    };

    // define the path
    let path = format!(
        "data/{}_t{}_r{}_{}.json",
        data_type, timespan, move_ratio, now
    );
    std::fs::write(path, measurement.to_string())?;
    Ok(())
}
