use hemera::measure_time;

// Invalid threshold format
#[measure_time(threshold = "invalid")]
fn invalid_threshold() {}

fn main() {}
