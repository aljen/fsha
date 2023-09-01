use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use rayon::prelude::*;
use sha2::{Digest, Sha512};

fn compute_sha512(m: &MultiProgress, pb: &ProgressBar, filename: &String) -> String {
    let file_size = std::fs::metadata(filename).unwrap().len();
    let mut file = std::fs::File::open(filename).unwrap();

    let spinner = m.add(ProgressBar::new(file_size).with_prefix(filename.clone()));

    spinner.set_style(
                        ProgressStyle::with_template(
                            "{spinner:.dim.bold.green} {prefix:.bold}▕{bar:40.yellow}▏{msg} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
                        )
                        .unwrap()
                        .progress_chars("█▇▆▅▄▃▂▁  ")
                    );

    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut hasher = Sha512::new();

    let _ = std::io::copy(&mut file, &mut spinner.wrap_write(&mut hasher));

    let hash = hasher.finalize();

    pb.inc(1);

    format!("{:x} {}", hash, filename)
}

fn main() {
    console::set_colors_enabled(true);

    let args_count = std::env::args().count() - 1;
    let files: Vec<String> = std::env::args().skip(1).collect();

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {prefix:.bold}▕{bar:40.green}▏{pos:>7}/{len:7} {wide_msg} ({eta})",
    )
    .unwrap()
    .progress_chars("█▇▆▅▄▃▂▁  ");

    let pb_tasks = m.add(ProgressBar::new(args_count as u64));
    pb_tasks.set_style(sty.clone());
    pb_tasks.set_message("finished");
    pb_tasks.enable_steady_tick(Duration::from_millis(100));

    let mut hashes: Vec<String> = files
        // .iter()
        .par_iter()
        .map(|file| {
            let m = m.clone();
            let pb_task = pb_tasks.clone();

            compute_sha512(&m, &pb_task, file)
        })
        .collect();

    m.clear().unwrap();

    hashes.sort();

    for hash in &hashes {
        println!("{}", hash);
    }
}
