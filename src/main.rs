use countmin::{CountMin, HCountMin};
use std::error::Error;
use std::fs::File;
use std::time::Instant;
use std::io::{BufReader, BufWriter, Write};
use clap::Parser;
use std::path::PathBuf;
use countmin::anograph::AnoGraph;


#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

#[derive(Debug, serde::Deserialize)]
struct Row {
    source: u64,
    target: u64,
    session: u64
}

#[derive(Debug, serde::Serialize)]
struct OutputRow {
    session: u64,
    score: f64,
}


fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments.
    let args = Args::parse();

    // Set up data reading.
    let f = File::open(args.file)?;
    let reader = BufReader::new(f);
    let mut csvreader = csv::ReaderBuilder::new().has_headers(false).from_reader(reader);
    
    // Initialize our anograph algorithm.
    let mut agraph = AnoGraph::<2, 32>::default();
    
    // Transform the raw csv data into our desired structure.
    let items = csvreader.deserialize().map(|result| {
        let row: Row = result.unwrap();
        (row.source, row.target, row.session)
    });

    // Set up our output stream.
    let out_file = File::create("output.csv")?;
    let out_buffer = BufWriter::new(out_file);
    let mut writer = csv::Writer::from_writer(out_buffer);
    let mut current_session: u64 = 1;
    let time = Instant::now();
    for (source, target, session) in items {
        if session != current_session {
            let score = agraph.score();
            let _ = writer.serialize(OutputRow {
                session: current_session,
                score,
            });
            //writer.flush()?;
            agraph.clear();
            current_session = session;
        }
        agraph.add_idx(source, target);
    }
    let score = agraph.score();
    let _ = writer.serialize(OutputRow {
        session: current_session,
        score,
    });
    writer.flush()?;
    let elapsed = time.elapsed();

    println!("Time: {:?}", elapsed);
    Ok(())
}

