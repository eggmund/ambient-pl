extern crate ears;
extern crate rand;
extern crate colored;

use colored::Colorize;
use ears::{Music, AudioController};
use rand::Rng;

use std::{time, fs, env, thread};
use std::thread::sleep;

mod info;

const TIME_WAIT_LOWER: u64 = 60;
const TIME_WAIT_UPPER: u64 = 300;

macro_rules! err_msg {
    ($x:expr) => {
        println!("ambient-pl: {}", $x.bold());
    }
}

fn play_file(file: &String, volume: f32) {
	let mut music = Music::new(file.as_str()).unwrap();
	music.set_volume(volume);
	music.play();

	println!("{}: {}", "Playing".bold(), file.bold().cyan());

	let loop_sleep_time = time::Duration::new(1, 0);
	while music.is_playing() {
		sleep(loop_sleep_time);
	}
}

fn is_audio_file(file_ext: &str) -> bool {
	match file_ext {
		"wav" | "flac" | "ogg" | "snd" | "raw" => true,
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
				sound_files.push(file_path);
			}
		}
	}

	sound_files
}

fn play_background(folder: String, recurse: bool) {
	let sound_files = get_file_list(folder, recurse, vec![]);

	if sound_files.len() <= 0 {
		err_msg!("No music files found. Try the '-r' flag for finding music in folders.");
		std::process::exit(1);
	}

	let mut rand_thr = rand::thread_rng();

	loop {
		play_file(&sound_files[rand_thr.gen_range(0, sound_files.len())], 1.0);
	}
}

fn parse_arguments() -> (String, bool, String) {  // Returns the folder, and whever to search recursively
	let args = env::args();
	let arg_list: Vec<String> = env::args().collect();
	let mut recurse = false;
	let mut folder = String::from("");
	let mut background = String::from("");

	for (i, a) in arg_list.iter().enumerate() {
		if a.contains("-") {
			match a.as_ref() {
				"-r" | "--recurse" => recurse = true,
				"--help" => info::print_help(),
				"-b" => {
					if i + 1 < arg_list.len() {
						background = arg_list[i + 1].clone();
					}
				},		// Background audio
					"" => (),
					_ => (),
			}
		} else {
			folder = a.to_string();
		}
	}

	if args.len() < 2 {
		err_msg!("Not enough arguments.");
		info::print_usage(true);
		std::process::exit(1);
	}

	if folder.as_str() == "" {
		err_msg!("Folder not specified.");
		info::print_usage(true);
		std::process::exit(1);
	}

	(folder, recurse, background)
}

fn main() {
	let (folder, recurse, background_audio) = parse_arguments();

	let background_enabled: bool = background_audio.as_str() != "";

	if background_enabled {
		thread::spawn(move || {
			play_background(background_audio, recurse);
		});
	}

	let sound_files = get_file_list(folder, recurse, vec![]);

	if sound_files.len() <= 0 {
		err_msg!("No music files found. Try the '-r' flag for finding music in folders.");
		std::process::exit(1);
	}

	let mut rand_thr = rand::thread_rng();

	let mut next_play_wait = time::Duration::new(0, 0);//time::Duration::new(rand_thr.gen_range(TIME_WAIT_LOWER, TIME_WAIT_UPPER), 0);

	loop {
		println!("Next in: {} seconds.", next_play_wait.as_secs().to_string().bold().blue());
		sleep(next_play_wait);
		play_file(&sound_files[rand_thr.gen_range(0, sound_files.len())], 0.5); // Blocks while music is playing
		next_play_wait = time::Duration::new(rand_thr.gen_range(TIME_WAIT_LOWER, TIME_WAIT_UPPER), 0);
	}
}
