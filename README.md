# rsb64

Base64 encoder and decoder written in Rust.

## build
```
cargo build --bin rsb64 --release
```

## run

### encode
`rsb64` reads content from file to encode and decode.

```
./rsb64 file_name
```

### decode
```
./rsb64 -decode file_name
```