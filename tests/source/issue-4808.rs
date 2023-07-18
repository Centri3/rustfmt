trait Trait {
    fn method(&self);
}

impl<F: Fn() -> T, T> Trait for F {
    fn method(&self) {}
}

fn main() {
    || .. .method();
}
