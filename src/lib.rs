extern crate nom;

mod parser;
mod parser_choices;
mod parser_delimiters;
mod parser_description;
mod parser_locale_country;
mod parser_locale_encoding;
mod templates;
use nom::error::convert_error;
use nom::error::VerboseError;
use nom::Err;

pub fn template_parser(
    i: &str,
) -> Result<
    (
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
    ),
    String,
> {
    match parser::template_parser::<VerboseError<&str>>(&i) {
        Ok((i, value)) => Ok((i, value)),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(convert_error(&i, e)),
        Err(Err::Incomplete(_e)) => Err(String::from("Incomplete")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
