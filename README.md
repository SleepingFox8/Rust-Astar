# Rust-Astar

This repository demonstrates the use of **the rust programming language** to concurrently pathfind to many destinations using the [Public-CivClassic-Nodes](https://github.com/SleepingFox8/Public-CivClassic-Nodes) dataset made for [TravelBot](https://github.com/SleepingFox8/AM-TravelBot)

## Running

```
cargo run --release
```

Will run the progrm and pathfind from one destination, to all known destinations. Stating that destinations's name and the time it will take (in minutes) to travel there. The final number is the time in seconds that it took to finish all pathfinding.
