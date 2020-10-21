use std::{
    fs::File,
    sync::{Arc, Mutex},
};

fn main() {
    let guard = pprof::ProfilerGuard::new(10000).unwrap();

    let secret_value = Arc::new(Mutex::new(5));

    let ref_a = secret_value.clone();

    let handle_a = std::thread::Builder::new()
        .name(String::from("lock_horder"))
        .spawn(move || {
            let mine = ref_a.as_ref().lock().unwrap();

            for i in 0..10 {
                println!("{}", i);
                std::thread::sleep(std::time::Duration::from_secs(15));
            }

            println!("{}", mine);
        });

    std::thread::sleep(std::time::Duration::from_secs(1));

    let handle_b = std::thread::Builder::new()
        .name(String::from("lock_requester"))
        .spawn(move || {
            let mine = secret_value.as_ref().lock().unwrap();

            std::thread::sleep(std::time::Duration::from_secs(15));
            println!("{}", mine);
        });

    handle_a.unwrap().join().unwrap();
    handle_b.unwrap().join().unwrap();

    match guard.report().build() {
        Ok(report) => {
            let file = File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
        }
        Err(e) => {
            println!("Failed to produce report, {}", e);
        }
    };

    println!("Hello, world!");
}
