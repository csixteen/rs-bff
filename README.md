# rs-bff

Simple [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter.

# Installing

```
$ make install
```

# Usage

```
Usage: rs-bff [OPTIONS]

Options:
  -c, --cells <CELLS>  Number of memory cells that the abstract machine will operate on [default: 30000]
  -f, --file <FILE>
  -h, --help           Print help
  -V, --version        Print version
```

```
$ rs-bff -f hello.bf
Hello, world!
```

You can also omit `-f`, in which case stdin can be used:

```
$ rs-bff < ./tests/hello.bf
Hello, world!
```

## Testing

```
cargo run -- -f tests/hello2.bf
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff -f tests/hello2.bf`
Hello, world!
cargo run -- -f tests/hello3.bf
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff -f tests/hello3.bf`
Hello World!
cargo run -- -f tests/hello.bf
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff -f tests/hello.bf`
Hello, world!
[test] finished!
```

## Limitations

It only works with 8-bit cells and only supports ASCII.

## Bugs

Please report any issues that you find. Or feel free to open a PR, it will be very welcome!

## LICENSE

See [LICENSE](https://github.com/csixteen/rs-bff/blob/master/LICENSE).
