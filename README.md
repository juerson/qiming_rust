# qiming_rust

这是一个起名辅助工具，可以遍历《通用规范汉字表》生成一个个"姓氏+名"的名字，从中挑出好听的名字。


### 特色

- 1、`名.txt`中，支持任意的中文文章。如果有特殊字符或不要的字符，需在`黑名单.txt`中添加，过滤掉不要的字符。

- 2、`黑名单.txt`中，将不要的字符过滤掉。

  注意：

  - 格式不作要求，可以横向摆放，挤在一起；
  - 只支持单个字符，不支持词语、成语、复姓过滤；
  - 要过滤的字符不全，还需要添加。

- 3、`姓氏.txt`中，支持生成单个姓氏或多个姓氏的名字。

  注意：每行只能放一个姓氏，程序不会检查姓氏是否合法。
  
### CLI

```
这是一个起名辅助工具，可以遍历《通用规范汉字表》生成一个个"姓氏+名"的名字，从中挑出好听的名字。

Usage: qiming_rust.exe [OPTIONS]

Options:
  -s, --surname-path <SURNAME_PATH>          姓氏文件路径 [default: 姓氏.txt]
  -n, --name-path <NAME_PATH>                名文件路径 [default: 名.txt]
  -b, --blacklist-path <BLACKLIST_PATH>      黑名单文件路径 [default: 黑名单.txt]
  -m, --max-write-number <MAX_WRITE_NUMBER>  每个文件最多生成多少个名字？(存放单字名字的文件除外) [default: 500000]
  -h, --help                                 Print help
  -V, --version                              Print version
```


### 运行结果示例：

```AMD Ryzen 7 7840HS
读取到：姓氏1个，字7491个！
预计每个姓氏可以生成：56122572个名字！
开始生成中.
总耗时: 13.1877696s
```