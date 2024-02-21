use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct ParserAPI {}

pub fn parser() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser_api() {
        let parse_result = ParserAPI::parse(Rule::sum, "1773 + 1362")
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        dbg!(parse_result);
    }

    #[test]
    fn test_pair() {
        let pairs = ParserAPI::parse(Rule::enclosed, "(..6472..) and more")
            .unwrap()
            .next()
            .unwrap()
            .into_inner();

        let numbers = pairs
            .clone()
            .map(|pair| str::parse(pair.as_str()).unwrap())
            .collect::<Vec<i32>>();
    }
}
