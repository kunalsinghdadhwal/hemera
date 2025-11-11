use hemera::measure_time;

#[measure_time]
fn fast_function() -> u32 {
    let mut sum = 0;
    for i in 0..1000 {
        sum += i;
    }
    sum
}

#[measure_time(name = "SlowOperation", level = "debug")]
fn slow_function() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[measure_time(threshold = "50ms")]
fn conditional_log(ms: u64) {
    // Only logs if execution time exceeds 50ms
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

#[measure_time(name = "CustomName", level = "debug", threshold = "10ms")]
fn all_options() {
    std::thread::sleep(std::time::Duration::from_millis(20));
}

fn main() {
    println!("Testing basic sync functions:\n");

    println!("1. Fast function:");
    let result = fast_function();
    println!("   Result: {}\n", result);

    println!("2. Slow function with custom name and debug level:");
    slow_function();
    println!();

    println!("3. Conditional logging (30ms - should NOT log):");
    conditional_log(30);
    println!();

    println!("4. Conditional logging (70ms - SHOULD log):");
    conditional_log(70);
    println!();

    println!("5. All options combined:");
    all_options();
}
