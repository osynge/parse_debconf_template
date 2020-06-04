extern crate nom;

mod parser;
mod parser_delimiters;
mod parser_description;
mod parser_locale_country;
mod parser_locale_encoding;
mod templates;
use nom::IResult;

pub fn template_parser(
    i: &str,
) -> IResult<
    &str,
    Vec<(
        &str,
        &str,
        &str,
        Option<(Vec<&str>, Vec<(&str, &str, Vec<&str>)>)>,
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    )>,
> {
    parser::template_parser(i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
