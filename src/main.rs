use std::{
    fs::{ self, File },
    io::{ BufWriter, Write },
    path::Path,
    sync::{ atomic::{ AtomicUsize, Ordering }, Arc },
    time::Instant,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;
use zhconv::zhconv;
use indexmap::IndexSet;
use crossbeam_channel::bounded;
use clap::{ error::ErrorKind, CommandFactory, Parser };

/// 这是一个起名辅助工具，可以遍历《通用规范汉字表》生成一个个"姓氏+名"的名字，从中挑出好听的名字。
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 姓氏文件路径
    #[arg(short, long, default_value = "姓氏.txt")]
    surname_path: String,

    /// 名文件路径
    #[arg(short, long, default_value = "名.txt")]
    name_path: String,

    /// 黑名单文件路径
    #[arg(short, long, default_value = "黑名单.txt")]
    blacklist_path: String,

    /// 每个文件最多生成多少个名字？(存放单字名字的文件除外)
    #[arg(short, long, default_value_t = 500_000)]
    max_write_number: usize,
}

fn main() {
    // 获取命令行参数
    let cli_command = Args::try_parse();
    match cli_command {
        Ok(args) => {
            let blocklist = load_blocklist(&args.blacklist_path);
            let surnames = load_filtered_words(&args.surname_path, &HashSet::default(), true);
            let names = load_filtered_words(&args.name_path, &blocklist, false);

            println!("读取到：姓氏{}个，字{}个！", surnames.len(), names.len());
            println!("预计每个姓氏可以生成：{}个名字！", names.len().pow(2) + names.len());
            print!("开始生成中");
            std::io::stdout().flush().unwrap();

            let file_counter = Arc::new(AtomicUsize::new(1));
            let dot_count = Arc::new(AtomicUsize::new(1));

            let dir_path = "Output";
            let output_path = Path::new(dir_path);
            clear_output_directory(output_path);
            fs::create_dir(output_path).expect("创建 output 文件夹失败");

            let start = Instant::now();

            // 使用通道并行写入
            let (sender, receiver) = bounded::<(Vec<String>, usize)>(
                rayon::current_num_threads() * 2
            );
            let writer_thread = std::thread::spawn(move || {
                while let Ok((buffer, file_id)) = receiver.recv() {
                    write_to_file(dir_path, &buffer, file_id);
                }
            });

            let surname_vec: Vec<String> = surnames.into_iter().collect();
            surname_vec.par_iter().for_each_with(sender.clone(), |s, surname| {
                let mut buffer = Vec::with_capacity(args.max_write_number);
                let mut single_names = Vec::with_capacity(names.len());

                for (i, name1) in names.iter().enumerate() {
                    // 处理1个字的名
                    single_names.push(format!("{}{}\n", surname, name1));

                    // 处理2个字的名
                    for name2 in &names {
                        buffer.push(format!("{}{}{}\n", surname, name1, name2));

                        if buffer.len() >= args.max_write_number {
                            let file_id = file_counter.fetch_add(1, Ordering::Relaxed);
                            s.send((buffer, file_id)).unwrap();
                            buffer = Vec::with_capacity(args.max_write_number);
                        }
                    }
                    // 降低刷新频率（动态打印记录，展示程序在执行中）
                    if i % 50 == 0 {
                        let count = dot_count.fetch_add(1, Ordering::Relaxed) % 10;
                        print!("\r\x1b[2K开始生成中{}", ".".repeat(count + 1));
                        std::io::stdout().flush().unwrap();
                    }
                }

                if !buffer.is_empty() {
                    let file_id = file_counter.fetch_add(1, Ordering::Relaxed);
                    s.send((buffer, file_id)).unwrap();
                }

                if !single_names.is_empty() {
                    let file_id = file_counter.fetch_add(1, Ordering::Relaxed);
                    s.send((single_names, file_id)).unwrap();
                }
            });

            drop(sender); // 关闭发送端
            writer_thread.join().unwrap();

            println!("\n总耗时: {:?}", start.elapsed());
        }
        Err(e) => {
            if
                e.kind() == ErrorKind::MissingRequiredArgument ||
                e.kind() == ErrorKind::InvalidValue
            {
                // 如果是因为缺少必需参数或无效值导致的错误，则显示帮助信息
                Args::command().print_help().unwrap();
            } else {
                // 其他类型的错误则正常打印错误信息
                e.print().unwrap();
            }
        }
    }
}

fn write_to_file(dir_path: &str, buffer: &[String], file_id: usize) {
    let file_name = format!("{}/No.{}.txt", dir_path, file_id);
    let file = File::create(&file_name).expect("无法创建文件");
    let mut writer = BufWriter::new(file);
    for line in buffer {
        write!(writer, "{}", line).expect("写入失败");
    }
}

fn clear_output_directory(dir: &Path) {
    if dir.exists() {
        rayon::scope(|s| {
            fs::read_dir(dir)
                .expect("读取 output 目录失败")
                .filter_map(Result::ok)
                .for_each(|entry| {
                    let path = entry.path();
                    s.spawn(move |_| {
                        if path.is_dir() {
                            fs::remove_dir_all(&path).ok();
                        } else {
                            fs::remove_file(&path).ok();
                        }
                    });
                });
        });
        fs::remove_dir_all(dir).expect("删除 output 目录失败");
    }
}

fn load_filtered_words(
    path: &str,
    blocklist: &HashSet<String>,
    is_surname_path: bool
) -> IndexSet<String> {
    if is_surname_path {
        fs::read_to_string(path)
            .expect("无法读取文件")
            .lines()
            .map(|line| zhconv(line, "zh-cn".parse().unwrap_or_default()))
            .filter(|word| !word.trim().is_empty())
            .filter(|word| !blocklist.contains(word.trim()))
            .collect::<IndexSet<_>>()
    } else {
        fs::read_to_string(path)
            .expect("无法读取文件")
            .lines()
            .map(|line| zhconv(line, "zh-cn".parse().unwrap_or_default()))
            .flat_map(|converted| {
                converted
                    .chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
            })
            .filter(|word| !word.trim().is_empty())
            .filter(|word| !blocklist.contains(word.trim()))
            .collect::<IndexSet<_>>()
    }
}

fn load_blocklist(path: &str) -> HashSet<String> {
    fs::read_to_string(path)
        .expect("无法读取'黑名单.txt'文件")
        .lines()
        .map(|line| zhconv(line, "zh-cn".parse().unwrap_or_default()))
        .flat_map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        })
        .filter(|word| !word.trim().is_empty())
        .collect()
}
