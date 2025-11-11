use hemera::measure_time;
use tokio::time::{Duration, sleep};

#[measure_time]
async fn fetch_data() -> String {
    sleep(Duration::from_millis(100)).await;
    "Data fetched successfully".to_string()
}

#[measure_time(name = "AsyncOperation")]
async fn process_data(data: String) -> usize {
    sleep(Duration::from_millis(50)).await;
    data.len()
}

#[measure_time(threshold = "30ms")]
async fn maybe_slow_async(ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    "Done"
}

#[measure_time(name = "ComplexAsync", level = "debug", threshold = "10ms")]
async fn complex_async() -> Result<i32, String> {
    sleep(Duration::from_millis(25)).await;
    Ok(42)
}

#[tokio::main]
async fn main() {
    println!("Testing async functions:\n");

    println!("1. Basic async function:");
    let data = fetch_data().await;
    println!("   Result: {}\n", data);

    println!("2. Async with custom name:");
    let len = process_data(data).await;
    println!("   Length: {}\n", len);

    println!("3. Conditional async (20ms - should NOT log):");
    let result = maybe_slow_async(20).await;
    println!("   Result: {}\n", result);

    println!("4. Conditional async (50ms - SHOULD log):");
    let result = maybe_slow_async(50).await;
    println!("   Result: {}\n", result);

    println!("5. Complex async with all options:");
    match complex_async().await {
        Ok(value) => println!("   Success: {}\n", value),
        Err(e) => println!("   Error: {}\n", e),
    }
}
