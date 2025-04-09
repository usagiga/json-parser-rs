use nom::character::complete::{alphanumeric0, char, multispace0};
use nom::sequence::{delimited};
use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;

fn parse_json(input: &str) -> IResult<&str, Vec<&str>> {
    // 空白文字と区切り文字たち
    // https://datatracker.ietf.org/doc/html/rfc8259#section-2
    let ws = multispace0;
    let begin_array = delimited(
        ws,
        tag("["),
        ws,
    );
    let begin_object = delimited(
        ws,
        tag("{"),
        ws,
    );
    let end_array = delimited(
        ws,
        tag("]"),
        ws,
    );
    let end_object = delimited(
        ws,
        tag("}"),
        ws,
    );
    let name_separator = delimited(
        ws,
        tag(":"),
        ws,
    );
    let value_separator = delimited(
        ws,
        tag(","),
        ws,
    );

    let mut parser = delimited(
        ws,
        delimited(
            begin_object,
            separated_list0(value_separator, alphanumeric0), // とりあえず alphanumeric0。本当は key と value を取り出したい
            end_object,
        ),
        ws,
    );

    parser.parse(input)
}

fn main() {
    // 入力
    // let input = r#"  {"hoge": "123","fuga": "456"}  "#;
    let input = r#"  {hoge  ,  fuga}  "#;
    // パース
    let parsed = parse_json(&input);
    println!("Parsed JSON: {:?}", parsed);
}