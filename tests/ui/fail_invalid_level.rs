use hemera::measure_time;

// Invalid level value
#[measure_time(level = "warn")]
fn invalid_level() {}

fn main() {}
