extern crate ears;
extern crate rand;
use ears::{Music, AudioController};
use rand::Rng;

use std::thread::sleep;
use std::{time, fs};

const time_wait_lower: u64 = 60;
const time_wait_upper: u64 = 300;

fn play_file(file: &String) {
    let mut music = Music::new(file.as_str()).unwrap();
    music.play();

    let loop_sleep_time = time::Duration::new(1, 0);
    while music.is_playing() {
        sleep(loop_sleep_time);
    }
}

fn get_file_list(folder: String) -> Vec<String> {
    let mut sound_files: Vec<String> = vec![];
    for p in fs::read_dir(folder.as_str()).unwrap() {
        sound_files.push(p.unwrap().path().display().to_string());
    }

    sound_files
}

fn get_next_play_time() -> time::Instant {
    let wait = rand::thread_rng().gen_range(time_wait_lower, time_wait_upper);
    println!("Next in: {} seconds.", wait);
    time::Instant::now() + time::Duration::new(wait, 0)
}

fn main() {
    let folder = String::from("/home/josh/minecraft_ost/");
    let sound_files = get_file_list(folder);

    let mut next_play_time = time::Instant::now();

    loop {
        if time::Instant::now() >= next_play_time {
            play_file(&sound_files[rand::thread_rng().gen_range(0, sound_files.len())]);
            next_play_time = get_next_play_time();
        }
    }
}
