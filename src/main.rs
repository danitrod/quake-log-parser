mod event_type;
mod parser;
mod report;

use parser::QuakeLogParser;
use std::io;

fn main() {
    let stdin = io::stdin();

    let mut p = QuakeLogParser::new();
    for (line_num, line) in stdin.lines().enumerate() {
        let line = line.expect(&format!("error parsing line {line_num} from stdin"));

        p.read_line(line)
            .map_err(|err| format!("error parsing line {line_num}: {err}"))
            .unwrap();
    }

    println!("{}", p.output_report_json());
}
