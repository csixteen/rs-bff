# rs-bff

I don't know Rust, so I've decided to write a simple [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter to practice.

---

I'm still pretty n00b in Rust and I'm currently still reading the [Rust book](https://doc.rust-lang.org/book/), which is pretty awesome btw. So apologies in advance if the code sucks. If you have any suggestions for improving the code, feel free to open a PR :D

# Installing

```
$ make install
```

# Usage

```
USAGE:
    rs-bff [OPTIONS] --source <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --source <FILE>    File with Brainfuck source code.
    -n, --num-cells <N>    Number of cells (default: 30,000)
```

```
$ rs-bff --source hello.bf
Hello, world!
```

Some of the tests were taken from [here](https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md) and [here](https://github.com/rdebath/Brainfuck)

## Testing

```
$ make test
cargo run -- --source tests/hello6.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rs-bff --source tests/hello6.bf`
Hello World!
cargo run -- --source tests/hello2.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff --source tests/hello2.bf`
Hello, world!
cargo run -- --source tests/hello3.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff --source tests/hello3.bf`
Hello World!
cargo run -- --source tests/add_two.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff --source tests/add_two.bf`
7cargo run -- --source tests/hello.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff --source tests/hello.bf`
Hello, world!
cargo run -- --source tests/hello4.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff --source tests/hello4.bf`
Hello World!
cargo run -- --source tests/hello5.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff --source tests/hello5.bf`
Hello, world!
[test] finished!
```

## Limitations

It only works with 8-bit cells and only supports ASCII.

## Bugs

Please report any issues that you find. Or feel free to open a PR, it will be very welcome!

## LICENSE

See [LICENSE](https://github.com/csixteen/rs-bff/blob/master/LICENSE).
