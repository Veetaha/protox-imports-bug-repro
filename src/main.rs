fn main() {
    dbg!(protox::compile(["foo/a.proto", "foo/b.proto"], ["./protobuf"]).unwrap());
}
