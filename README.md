# FizzBuzz

A fast fizzbuzz implementation with light use of SSE2. Linux only. Outputs at ~2.5GiB/s on my laptop, see below.

This implementation caps out at 10^16, or on the order of 2^53.

## Performance

For reference, on my computer:
* Simple FizzBuzz implementation yields 29MiB/s
* [Aiden4's FizzBuzz](https://codegolf.stackexchange.com/a/217455) in Rust pipes about ~1.7GiB/s
* The legendary [FizzBuzz implementation by ais253](https://codegolf.stackexchange.com/a/236630) does ~20GiB/s

With the command:
```sh
taskset 3 sh -c "RUSTFLAGS=\"-C opt-level=3 -C target-cpu=native\" cargo run --release" | taskset 4 pv > /dev/null
```
I get 100GiB in 40 seconds, so a sustained 2.5GiB/s (it increases as the numbers get bigger though).
You may need to play around with the CPU numbers on your machine.
