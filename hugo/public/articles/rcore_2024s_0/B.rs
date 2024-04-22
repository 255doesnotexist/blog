use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

fn main() {
    // create multiple threads and execute tasks asynchronously
    let thread_handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} started", i);
                thread::sleep(Duration::from_secs(i as u64));

                // access shared variable using a mutex
                let shared_data = Arc::new(Mutex::new(0));
                { // using lock here 
                    let mut data = shared_data.lock().unwrap();
                    *data += i;
                    println!("Thread {} updated shared data: {}", i, *data);
                }

                // asynchrony and persistence: write data to a file
                let file_path = format!("output_{}.txt", i);
                let mut file = File::create(file_path).await.unwrap();
                let message = format!("Hello from thread {}", i);
                file.write_all(message.as_bytes()).await.unwrap();

                println!("Thread {} finished", i);
            })
        })
        .collect();

    for handle in thread_handles {
        handle.join().unwrap();
    }
}