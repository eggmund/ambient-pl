extern crate ears;
extern crate rand;
extern crate colored;

use colored::Colorize;
use ears::{Music, AudioController};
use rand::Rng;

use std::thread::sleep;
use std::{time, fs, env};

const TIME_WAIT_LOWER: u64 = 60;
const TIME_WAIT_UPPER: u64 = 300;

fn play_file(file: &String) {
    let mut music = Music::new(file.as_str()).unwrap();
    music.play();

    println!("{}: {}", "Playing".bold(), file.bold().cyan());

    let loop_sleep_time = time::Duration::new(1, 0);
    while music.is_playing() {
        sleep(loop_sleep_time);
    }
}

fn is_audio_file(file_ext: &str) -> bool {
    match file_ext {
        "wav" => true,
        "flac" => true,
        _ => false
    }
}

fn get_file_list(folder: String, mut sound_files: Vec<String>) -> Vec<String> {
    for p in fs::read_dir(folder.as_str()).unwrap() {
        let p_unw = p.unwrap();
        let file_path = p_unw.path().display().to_string();

        if p_unw.file_type().unwrap().is_dir() {
            sound_files = get_file_list(file_path, sound_files);
        } else {
            let parts: Vec<&str> = file_path.split(".").collect();
            if is_audio_file(parts.last().unwrap()) {
                println!("Adding: {}", file_path);
                sound_files.push(file_path);
            }
        }
    }

    sound_files
}

fn get_next_play_time() -> time::Instant {
    let wait = rand::thread_rng().gen_range(TIME_WAIT_LOWER, TIME_WAIT_UPPER);
    println!("Next in: {} seconds.", wait.to_string().bold().blue());
    time::Instant::now() + time::Duration::new(wait, 0)
}

fn print_usage() {
    println!("{}: ambient-pl {}", "Usage".bold(), "[directory]".bold().yellow());
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("ambient-pl: {}", "Incorrect number of arguments.".bold());
        print_usage();
        std::process::exit(1);
    }

    let folder = args.nth(1).unwrap();
    let sound_files = get_file_list(folder, vec![]);

    let mut next_play_time = time::Instant::now();

    loop {
        if time::Instant::now() >= next_play_time {
            play_file(&sound_files[rand::thread_rng().gen_range(0, sound_files.len())]);
            next_play_time = get_next_play_time();
        }
    }
}
