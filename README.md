# `rsb64`

Base64 encoder and decoder written in Rust.

<img src="https://gitlab.com/jmaczan/rsb64/-/raw/main/rsb64.png" max-height="300">

## build
```
cargo build --bin rsb64 --release
```

Output binary file is available under `./target/release/rsb64`

## run
In order to encode or decode, `rsb64` reads content from a file.

### encode

```
./rsb64 file_name
```

### decode
```
./rsb64 -decode file_name
```

## license
GPL2 for non-commercial use.

For commercial usage, [please contact me via email](mailto:jedrzejpawel@maczan.pl).

## author

Made in [Poland](https://en.wikipedia.org/wiki/Poland) in 2022 by [Jedrzej Pawel Maczan](https://maczan.pl/)