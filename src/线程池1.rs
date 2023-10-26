use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::sync::Mutex;
use threadpool::ThreadPool;

static LAST_NAMES: &[&str] = &[
    "张",
];

fn main() {
    let file = File::open("通用规范汉字表.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let first_names: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|word| word.trim().to_string())
        .collect();

    let contains_vec = vec!["死", "囚"];

    let file = Arc::new(Mutex::new(File::create("output.txt").unwrap()));

    let last_names: Vec<String> = LAST_NAMES.iter().map(|&s| s.to_owned()).collect();

    for last_name in last_names.iter() {
        let file = Arc::clone(&file);
        let contains_vec = contains_vec.clone();
        let last_name = last_name.clone();
        let thread_pool = ThreadPool::new(200); // Adjust the number of threads in the pool

        for first_name in &first_names {
            if first_name != last_name.as_str() && !contains_vec.contains(&first_name.as_str()) {
                let file_clone = Arc::clone(&file);
                let name1 = format!("{}{}", last_name, first_name.clone());
                println!("{}", name1);

                thread_pool.execute(move || {
                    let mut file = file_clone.lock().unwrap();
                    file.write_all(format!("{}\n", name1).as_bytes()).unwrap();
                    file.flush().expect("Failed to flush buffer");
                });

                for two_name in &first_names {
                    if two_name != last_name.as_str() && !contains_vec.contains(&two_name.as_str()) {
                        let file_clone = Arc::clone(&file);
                        let name2 = format!("{}{}{}", last_name, first_name.clone(), two_name);
                        println!("{}", name2);

                        thread_pool.execute(move || {
                            let mut file = file_clone.lock().unwrap();
                            file.write_all(format!("{}\n", name2).as_bytes()).unwrap();
                            file.flush().expect("Failed to flush buffer");
                        });
                    }
                }
            }
        }
        thread_pool.join();
    }

    
}
