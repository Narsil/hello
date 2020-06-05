#[link(name = "foo")]
extern "C" {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
