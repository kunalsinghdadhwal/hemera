use hemera::measure_time;

#[measure_time]
fn generic_function<T: Clone>(value: T) -> T {
    value.clone()
}

#[measure_time]
async fn generic_async<T: std::fmt::Display>(value: T) -> String {
    format!("{}", value)
}

#[measure_time(name = "GenericWithAttrs")]
fn generic_with_attrs<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

fn main() {}
