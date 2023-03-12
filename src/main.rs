extern crate bdf;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let file_name = env::args()
        .nth(1)
        .expect("フォント名を表す引数を与えてください");

    let output_file_name = env::args()
        .nth(2)
        .expect("出力用ファイル名を表す引数を与えてください");

    let mut font = File::open(&file_name).expect("BDF ファイルを開けませんでした");

    let mut output_file =
        File::create(output_file_name).expect("書き込み用ファイルを作成できませんでした");

    let mut contents = String::new();

    font.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.lines();

    let mut reader = csv::Reader::from_path("./JISX0208.CSV")
        .expect("JISX0208.CSV 変換テーブルファイルを開けませんでした");

    let mut convert_map: HashMap<String, String> = HashMap::new();

    for record in reader.records() {
        let record = record.unwrap();
        convert_map.insert(record[0].to_string(), record[1].to_string());
    }

    // ここからテキストの置換
    for line in lines {
        if !line.starts_with("ENCODING") {
            writeln!(output_file, "{}", line).unwrap();
            continue;
        }

        let splitted: Vec<&str> = line.split_whitespace().collect();
        let jisx0208_codepoint = splitted[1];
        let jisx0208_codepoint =
            u32::from_str_radix(jisx0208_codepoint, 10).expect("Parse に失敗しました");
        let replaced = format!("0x{:x}", jisx0208_codepoint);

        // JIS X 0208 の Codepoint に対応する Unicode Codepoint を検索する
        let unicode_codepoint = convert_map.get(&replaced);

        let unicode_codepoint = match unicode_codepoint {
            Some(codepoint) => u32::from_str_radix(&codepoint.replace("0x", ""), 16).unwrap(),
            None => {
                println!("JIS X 0208 codepoint {} は、対応する Unicode codepoint が見つからないためスキップします", replaced);
                continue;
            }
        };

        println!("Replaced codepoint: {:x}", unicode_codepoint);

        writeln!(output_file, "ENCODING {}", unicode_codepoint).unwrap();
    }
}
