use core::time;
use std::thread;

pub fn watch_files(files: &[&str]) {
    for i in 0..files.len() {
        println!("{}. {}", i + 1, files[i]);
    }

    loop {
        for file in files.iter() {
            watch_file(file);
        }
        let time_millis = time::Duration::from_millis(1_000);
        thread::sleep(time_millis);
    }
}

fn watch_file(file: &str) {
    println!("Watching file: {}", file);
}
