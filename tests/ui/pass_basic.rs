use hemera::hemera;

#[hemera]
fn simple_sync() -> i32 {
    42
}

#[hemera]
async fn simple_async() -> String {
    "hello".to_string()
}

#[hemera(name = "Custom")]
fn with_name() {}

#[hemera(level = "debug")]
fn with_level() {}

#[hemera(threshold = "10ms")]
fn with_threshold() {}

#[hemera(name = "Test", level = "debug", threshold = "5ms")]
fn all_attrs() {}

fn main() {}
