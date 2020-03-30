# rs-bff

Simple [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter.

# Installing

```
$ make install
```

# Usage

```
USAGE:
    rs-bff [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --num-cells <N>    Number of cells (default: 30,000)
```

```
$ rs-bff hello.bf
Hello, world!
```

Some of the tests were taken from [here](https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md) and [here](https://github.com/rdebath/Brainfuck)

## Testing

```
$ make test
cargo run tests/hello6.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rs-bff tests/hello6.bf`
Hello World!
cargo run tests/hello2.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff tests/hello2.bf`
Hello, world!
cargo run tests/hello3.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff tests/hello3.bf`
Hello World!
cargo run tests/add_two.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff tests/add_two.bf`
7cargo run tests/hello.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff tests/hello.bf`
Hello, world!
cargo run tests/hello4.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff tests/hello4.bf`
Hello World!
cargo run tests/hello5.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff tests/hello5.bf`
Hello, world!
[test] finished!
```

## Limitations

It only works with 8-bit cells and only supports ASCII.

## Bugs

Please report any issues that you find. Or feel free to open a PR, it will be very welcome!

## LICENSE

See [LICENSE](https://github.com/csixteen/rs-bff/blob/master/LICENSE).
