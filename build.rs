// build.rs

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/hello.cpp")
        .compile("foo");
}
