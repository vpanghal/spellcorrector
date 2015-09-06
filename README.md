# Spelling Corrector
This is [Rust] implementation of [Norvig's spelling corrector]. All theory and
other language implementations of spell corrector can be found on his website.
My objective was to explore the shiny new lanaguage.

In order to run spell corrector

```sh
$ cargo build --release
```

This will start http rest server listening on port 3000 based on [Iron] framework
```sh
$ ./target/release/spell_corrector
```

Supported rest APIs

```sh
$ curl http://localhost:3000/ping
$ curl -X POST -d 'Korrect' http://localhost:3000/correct
```

Integration tests can be run with

```sh
$ cargo test
```

[Rust]: http://www.rust-lang.org/
[Iron]: https://github.com/iron/iron
[Norvig's spelling corrector]:http://norvig.com/spell-correct.html
