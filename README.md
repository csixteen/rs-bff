# rs-bff

I don't know Rust, so I've decided to make a port of my [original](https://github.com/csixteen/py-bff) Python Brainfuck interpreter to Rust. I guess this sort of became my toy project for when I want to solidify some concepts while learning a new language. I've done the same for [Go](https://github.com/csixteen/go-bff) and I'll probably do the same for other languages.

---

I'm still pretty n00b in Rust and I'm currently still reading the [Rust book](https://doc.rust-lang.org/book/), which is pretty awesome btw. So apologies in advance if the code sucks. If you have any suggestions for improving the code, feel free to open a PR :D

## How to run

```
$ cargo run hello.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rs-bff hello.bf`
Hello, world!
Hello, world!
```

## Limitations

There are several! But I'd say that one of the main limitations is that the interpreter assumes the source code to be correct. For instance if you have square brackets without the matching opening of closing square bracket, then you will end up with very unexpected results.
