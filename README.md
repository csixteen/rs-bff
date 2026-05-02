# rs-bff

Simple [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter.

## Installing

```
$ make install
```

## Usage

```
Usage: bff [OPTIONS]

Options:
  -c, --cells <CELLS>  Number of memory cells that the abstract machine will operate on [default: 30000]
  -f, --file <FILE>
  -h, --help           Print help
  -V, --version        Print version
```

```
$ bff -f ./tests/hello.bf
Hello, world!
```

You can also omit `-f`, in which case stdin can be used:

```
$ bff < ./tests/hello.bf
Hello, world!
```

You can also run the interpreter with a TUI:

```
$ bff-tui -f ./tests/hello.bf
```

## Testing

```
make test
```

## Limitations

It only works with 8-bit cells and only supports ASCII.

## Bugs

Please report any issues that you find. Or feel free to open a PR, it will be very welcome!

## LICENSE

See [LICENSE](https://github.com/csixteen/rs-bff/blob/master/LICENSE).
