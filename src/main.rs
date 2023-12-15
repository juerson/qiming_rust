use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use std::time::{Duration, Instant};

fn main() {
	// 初始化上一次提醒的时间为程序启动的时间
    let mut last_alert_time = Instant::now();
	// 记录程序运行了多少分钟
	let mut minute_count:u32 = 0;
    let file = File::open("通用规范汉字表.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let first_names: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|word| word.trim().to_string())
        .collect();
	// 排除一些消极的字，除了下面的字外，您还可以添加更多不需要的汉字
    let contains_vec = vec!["死", "囚","尸","颓","负","丧","废","悲","丐","丑","丸","亡","亵","棺","椁","傻","癌","奸","娼","妓","娠","娩","危","奠","墓","匕","咬","吃","吊","厕","扁","劓","剥","剜","剁","刺","刹","剃"];
	// 您的姓氏
    let last_name = "赵";
	
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
	
	// 记录生成名字的数量
    let mut no_numbers:u32 = 1;
	
	// 两个字的名字（姓氏+一个字）
    thread_pool1.install(|| {
        rayon::scope(|s| {
            s.spawn(|_| {
                for first_name in &first_names {
                    if first_name == last_name || contains_vec.contains(&first_name.as_str()) {
                        continue;
                    }
                    let first_name_clone = first_name.clone();
					let current_time_str = get_current_time();
                    let name1 = format!("{}{}", last_name, first_name_clone);
					
					// 检查是否已经过了一分钟
					if last_alert_time.elapsed() >= Duration::from_secs(60) {
						// 提醒又一次过去一分钟
						println!("{} 第{}个名字：{} (程序已经运行{}分钟！)", current_time_str, no_numbers, name1, minute_count);
						minute_count += 1;
						no_numbers += 1;
						// 更新上一次提醒的时间
						last_alert_time = Instant::now();
					} else {
						println!("{} 第{}个名字：{} (程序已经运行{}分钟！)", current_time_str, no_numbers, name1, minute_count);
						no_numbers += 1;
					}
					
                    let mut output1_file = output1_file.lock().unwrap();
                    output1_file
                        .write_all(format!("{}\n", name1).as_bytes())
                        .unwrap();
                    output1_file.flush().expect("Failed to flush buffer");
                }
            });
        });
    });
	
    let mut file_number:u32 = 1;
	// 三个字的名字（姓氏+两个字）
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
						let current_time_str = get_current_time();
                        let name2 = format!("{}{}{}", last_name, first_name_clone, two_name_clone);
                        						
						// 检查是否已经过了一分钟
						if last_alert_time.elapsed() >= Duration::from_secs(60) {
							// 提醒又一次过去一分钟
							println!("{} 第{}个名字：{} (程序已经运行{}分钟！)", current_time_str, no_numbers, name2, minute_count);
							minute_count += 1;
							no_numbers += 1;
							// 更新上一次提醒的时间
							last_alert_time = Instant::now();
						} else {
							println!("{} 第{}个名字：{} (程序已经运行{}分钟！)", current_time_str, no_numbers, name2, minute_count);
							no_numbers += 1;
						}
						
                        let output2_file = create_output_file(file_number);
                        let mut output2_file = output2_file.lock().unwrap();
                        output2_file
                            .write_all(format!("{}\n", name2).as_bytes())
                            .unwrap();
                        output2_file.flush().expect("Failed to flush buffer");
                        let mut lines_written2 = lines_written2.lock().unwrap();
                        *lines_written2 += 1;
                        // println!("{}",*lines_written2);
                        if *lines_written2 >= 100000 {
                            // let mut file_number = file_number.lock().unwrap();
                            file_number += 1; // 修改 file_number
                            *lines_written2 = 0;
                        }
                    }
                }
            });
        });
    });
}

fn create_output_file(file_number: u32) -> Arc<Mutex<File>> {
    let file_name = format!("output{}.txt", file_number);
    let new_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_name)
        .expect("Failed to create or open output file");

    Arc::new(Mutex::new(new_file))
}

// 获取当前的时间，并格式化时间（显示人类容易读取的时间）
fn get_current_time() -> String {
    let current_time = Local::now();
    current_time.format("%Y-%m-%d %H:%M:%S").to_string()
}