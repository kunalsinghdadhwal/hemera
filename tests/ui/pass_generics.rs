use hemera::hemera;

#[hemera]
fn generic_function<T: Clone>(value: T) -> T {
    value.clone()
}

#[hemera]
async fn generic_async<T: std::fmt::Display>(value: T) -> String {
    format!("{}", value)
}

#[hemera(name = "GenericWithAttrs")]
fn generic_with_attrs<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

fn main() {}
