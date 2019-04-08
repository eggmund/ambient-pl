extern crate ears;
extern crate rand;
extern crate colored;

use colored::Colorize;
use ears::{Music, AudioController};
use rand::Rng;

use std::thread::sleep;
use std::{time, fs, env};

mod info;

const TIME_WAIT_LOWER: u64 = 60;
const TIME_WAIT_UPPER: u64 = 300;

macro_rules! err_msg {
    ($x:expr) => {
        println!("ambient-pl: {}", $x.bold());
    }
}

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

fn get_file_list(folder: String, recurse: bool, mut sound_files: Vec<String>) -> Vec<String> {
    for p in fs::read_dir(folder.as_str()).unwrap() {
        let p_unw = p.unwrap();
        let file_path = p_unw.path().display().to_string();

        if recurse && p_unw.file_type().unwrap().is_dir() {
            sound_files = get_file_list(file_path, recurse, sound_files);
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

fn parse_arguments() -> (String, bool) {  // Returns the folder, and whever to search recursively
    let args = env::args();
    let arg_list: Vec<String> = env::args().collect();
    let mut recurse = false;
    let mut folder = String::from("");

    for a in arg_list.iter() {
        if a.contains("-") && !a.contains("/") {
            match a.as_ref() {
                "-r" | "--recurse" => {recurse = true; println!("egg")},
                "--help" => info::print_help(),
                "" => (),
                _ => (),
            }
        } else {
            folder = a.to_string();
        }
    }

    if args.len() < 2 {
        err_msg!("Not enough arguments.".bold());
        info::print_usage(true);
        std::process::exit(1);
    }

    if folder.as_str() == "" {
        err_msg!("Folder not specified.".bold());
        info::print_usage(true);
        std::process::exit(1);
    }

    return (folder, recurse)
}

fn main() {
    let (folder, recurse) = parse_arguments();

    let sound_files = get_file_list(folder, recurse, vec![]);

    let main_loop_sleep_time = time::Duration::new(1, 0);

    let mut next_play_wait = time::Duration::new(rand::thread_rng().gen_range(TIME_WAIT_LOWER, TIME_WAIT_UPPER), 0);

    loop {
        println!("Next in: {} seconds.", next_play_wait.as_secs().to_string().bold().blue());
        sleep(next_play_wait);
        play_file(&sound_files[rand::thread_rng().gen_range(0, sound_files.len())]); // Blocks while music is playing
        next_play_wait = time::Duration::new(rand::thread_rng().gen_range(TIME_WAIT_LOWER, TIME_WAIT_UPPER), 0);
    }
}
