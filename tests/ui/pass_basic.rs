use hemera::measure_time;

#[measure_time]
fn simple_sync() -> i32 {
    42
}

#[measure_time]
async fn simple_async() -> String {
    "hello".to_string()
}

#[measure_time(name = "Custom")]
fn with_name() {}

#[measure_time(level = "debug")]
fn with_level() {}

#[measure_time(threshold = "10ms")]
fn with_threshold() {}

#[measure_time(name = "Test", level = "debug", threshold = "5ms")]
fn all_attrs() {}

fn main() {}
