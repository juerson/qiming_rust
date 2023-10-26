use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

fn main() {
    let file = File::open("通用规范汉字表.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let first_names: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|word| word.trim().to_string())
        .collect();

    let contains_vec = vec!["死", "囚"];

    let last_name = "张";

    let thread_pool1 = rayon::ThreadPoolBuilder::new()
        .num_threads(200)
        .build()
        .unwrap();
    let thread_pool2 = rayon::ThreadPoolBuilder::new()
        .num_threads(200)
        .build()
        .unwrap();

    let lines_written2 = Arc::new(Mutex::new(0));

    let output1_file = Arc::new(Mutex::new(
        File::create("output.txt").expect("Failed to create output file"),
    ));

    
    thread_pool1.install(|| {
        rayon::scope(|s| {
            s.spawn(|_| {
                for first_name in &first_names {
                    if first_name == last_name || contains_vec.contains(&first_name.as_str()) {
                        continue;
                    }
                    let first_name_clone = first_name.clone();
                    let name1 = format!("{}{}", last_name, first_name_clone);
                    println!("{}", name1);
                    let mut output1_file = output1_file.lock().unwrap();
                    output1_file
                        .write_all(format!("{}\n", name1).as_bytes())
                        .unwrap();
                    output1_file.flush().expect("Failed to flush buffer");
                }
            });
        });
    });

    let file_number = Arc::new(Mutex::new(1));
    let output2_file = create_output_file(&file_number.clone());

    thread_pool2.install(|| {
        rayon::scope(|s| {
            s.spawn(|_| {
                for first_name in &first_names {
                    if first_name == last_name || contains_vec.contains(&first_name.as_str()) {
                        continue;
                    }
                    let first_name_clone = first_name.clone();

                    for two_name in &first_names {
                        if two_name == last_name || contains_vec.contains(&two_name.as_str()) {
                            continue;
                        }
                        let two_name_clone = two_name.clone();
                        let name2 = format!("{}{}{}", last_name, first_name_clone, two_name_clone);
                        println!("{}", name2);
                        let mut output2_file = output2_file.lock().unwrap();
                        output2_file
                            .write_all(format!("{}\n", name2).as_bytes())
                            .unwrap();
                        output2_file.flush().expect("Failed to flush buffer");
                        let mut lines_written2 = lines_written2.lock().unwrap();
                        *lines_written2 += 1;

                        if *lines_written2 >= 10000 {
                            let mut file_number = file_number.lock().unwrap();
                            *file_number += 1;
                            *lines_written2 = 0;
                        }
                    }
                }
            });
        });
    });
}

fn create_output_file(file_number: &Arc<Mutex<u32>>) -> Arc<Mutex<File>> {
    let file_number = file_number.lock().unwrap();
    println!("Creating output file {}", *file_number);
    let file_name = format!("output{}.txt", *file_number);
    let new_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_name)
        .expect("Failed to create or open output file");

    Arc::new(Mutex::new(new_file))
}
