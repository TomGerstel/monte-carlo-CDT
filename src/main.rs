mod universe;
use serde_json::json;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::SystemTime;
use structopt::StructOpt;

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

    /// Option to choose between doing a measurement or outputing mesh
    #[structopt(short = "v", long)]
    visualisation: bool,

    /// Option to choose outputing std only
    #[structopt(short = "s", long)]
    output_std: bool,

    /// Probability of performing a shard move for a single Markov chain step
    /// in the equilibration phase
    #[structopt(short = "e", long, default_value = "0.5")]
    // TODO: adjust default value when we know optimal value
    move_ratio_eq: f32,

    /// Length of equilibration phase in sweeps (bake-in)
    #[structopt(short = "b", long, default_value = "0")]
    eq_sweeps: usize,

    /// Number of sweeps inbetween measurements (pause)
    #[structopt(short = "p", long, default_value = "1.0")]
    pause: f32,

    #[structopt(short = "o", long)]
    output_folder: String,
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
    let move_ratio_eq = opt.move_ratio_eq;
    let eq_sweeps = opt.eq_sweeps;
    let output_folder = opt.output_folder;
    let visualisation = opt.visualisation;
    let output_std = opt.output_std;

    let sweep = 2 * timespan * length;

    // check parameters
    assert!(
        (0.0..=1.0).contains(&move_ratio),
        "given move ratio ({}) is outside valid range [0.0, 1.0]",
        move_ratio
    );
    assert!(
        (0.0..=1.0).contains(&move_ratio_eq),
        "given move ratio ({}) is outside valid range [0.0, 1.0]",
        move_ratio_eq
    );

    // determine the number of timesteps between measurements
    let pause = match is_measurement {
        true => (opt.pause * sweep as f32) as usize,
        false => 1,
    };
    assert!(
        pause > 0,
        "the given value {} for pause results in no MC steps",
        pause
    );

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

    // create the filename and path strings
    let name = format!(
        "{}_t{}_l{}_n{}_r{}_{}",
        data_type, timespan, length, n_save, move_ratio, now
    );
    let data_path = format!("{}/{}.csv", output_folder, name);
    let config_path = format!("{}/{}.json", output_folder, name);

    if visualisation {
        // big bang
        let mut universe = universe::Universe::new(timespan, length);

        for _ in 0..(n_save * sweep) {
            universe.mcmc_step(move_ratio_eq);
        }

        write_triangulation_mesh(&universe, &format!("{}/mesh_{}.csv", output_folder, name))
    } else {
        // put everything in json format (TODO: No need to do this, serde can do this from Opt)
        let measurement = json!({
            "name": name,
            "is_measurement": is_measurement,
            "timespan": timespan,
            "length": length,
            "move_ratio": move_ratio,
            "n_save": n_save,
            "pause": pause,
            "move_ratio_eq": move_ratio_eq,
            "eq_sweeps": eq_sweeps,
        });

        std::fs::write(config_path, measurement.to_string())?;
    

        // big bang
        let mut universe = universe::Universe::new(timespan, length);

        // do equilibration phase if required
        if is_measurement {
            for _ in 0..(eq_sweeps * sweep) {
                universe.mcmc_step(move_ratio_eq);
            }
        }

        // open buffer to write into
        let mut output = BufWriter::new(File::create(&data_path).unwrap());

        // measurement phase
        for _ in 0..n_save {
            for _ in 0..pause {
                universe.mcmc_step(move_ratio);
            }
            // do the measurement
            let origin = fastrand::usize(0..sweep);
            let length_profile = universe.length_profile(origin);

            // write to file
            if output_std {
                writeln!(output, "{}, ", length_profile.stdev())?;
            } else {
                writeln!(output, "{}", length_profile)?;
            }
        }

        // flush buffer
        output.flush()
    }
}

fn write_triangulation_mesh(universe: &universe::Universe, data_path: &str) -> std::io::Result<()> {
    let mesh = universe.torus_triangle_coordinates();

    
    let mut output = BufWriter::new(File::create(data_path).unwrap());
    for triangle in mesh {
        writeln!(output, "{},{},{}", triangle.0, triangle.1, triangle.2)?;
    }
    output.flush()
}