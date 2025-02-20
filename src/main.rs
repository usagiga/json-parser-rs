use nom::character::complete::char;
use nom::sequence::delimited;
use nom::{IResult, Parser};
use nom::bytes::complete::take_while;

fn parse_json(input: &str) -> IResult<&str, &str> {
    // パース
    let mut parser = delimited(
        char('{'),
        take_while(|c| c != '}'),
        char('}'),
    );

    parser.parse(input)
}

fn main() {
    // 空のJSON文字列
    let input = "{\"hoge\": \"fuga\"}";
    // パース
    let parsed = parse_json(&input);
    println!("Parsed JSON: {:?}", parsed);
}