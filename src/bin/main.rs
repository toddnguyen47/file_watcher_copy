use file_watcher_copy::file_watcher;

fn main() {
    let mut vec1 = Vec::new();
    vec1.push("hello");
    vec1.push("world");
    file_watcher::watch_files(vec1.as_slice());
}
