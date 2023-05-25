# Reproduction of the import resolution bug in `protox`

If you run this code with

```
cargo run
```

then it will compile and run without an error, even though the protobuf declarations residing in `protobuf/` directory have a bug. The file `b.proto` uses the type defined in `a.proto`, but to be able to do this it must first import the file via

```proto
import "foo/a.proto"
```

`protox` doesn't give a compile error, but `protoc` does. If you run the following `protoc` command you'll see an error.

```bash
$ protoc foo/a.proto foo/b.proto -I ./protobuf --ruby_out=./ruby-out
foo/b.proto:7:14: "foo.a" seems to be defined in "foo/a.proto", which is not imported by "foo/b.proto".  To use it here, please add the necessary import.
```
