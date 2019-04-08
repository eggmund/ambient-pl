# ambient-pl
An audio player that runs in the background and randomly picks tracks from a folder to play at random times.

Inspired by the way Minecraft music randomly fades in while playing.

## How to use it

Play songs from a folder:

```bash
ambient-pl -r <folder_here>
```

`-r` is optional, and is for recursively adding music.

## Installation

Change directory to where you cloned the repository and run:
```bash
cargo build
```

And find the executable in `target/debug/ambient-pl`. You can then move that to your PATH and run it in the terminal (if you want to).

## Dependencies

- ears (for playing the audio files): https://github.com/jhasse/ears
- colored (for coloured terminal output): https://github.com/mackwic/colored
- rand
