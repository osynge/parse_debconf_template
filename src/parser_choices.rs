use crate::parser_delimiters::{
    delimiter_description_locale, delimiter_key_value, delimiter_line,
    delimiter_locale_country_encoding, delimiter_package_section,
};
use crate::parser_locale_country::locale_country;
use crate::parser_locale_encoding::locale_encoding;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::complete;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn key_choices<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("Choices")(i)
}

fn is_choices_char(c: char) -> bool {
    match c {
        '\n' => false,
        ',' => false,
        _ => true,
    }
}

fn line_parser_choices_default<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> {
    let (i, _) = key_choices(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, choices) = separated_list1(tag(", "), take_while1(is_choices_char))(i)?;
    Ok((i, choices))
}

fn line_parser_choices_locales<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&str, (&str, &str, Vec<&str>), E> {
    let (i, _) = key_choices(i)?;
    let (i, _) = delimiter_description_locale(i)?;
    let (i, country) = locale_country(i)?;
    let (i, _) = delimiter_locale_country_encoding(i)?;
    let (i, encoding) = locale_encoding(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, choices) = separated_list1(tag(", "), take_while1(is_choices_char))(i)?;
    Ok((i, (country, encoding, choices)))
}

fn line_parser_choices_locales_all<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&str, Vec<(&'a str, &'a str, Vec<&'a str>)>, E> {
    separated_list1(tag("\n"), line_parser_choices_locales)(i)
}

pub(crate) fn line_parser_choices_all_locales<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (Vec<&'a str>, Vec<(&'a str, &'a str, Vec<&'a str>)>), E> {
    let mut tpl = tuple((
        line_parser_choices_default,
        delimiter_line,
        line_parser_choices_locales_all,
    ));
    let (i, (line, _, details)) = tpl(i)?;
    Ok((i, (line, details)))
}

pub(crate) fn line_parser_choices_all_default_only<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&str, (Vec<&str>, Vec<(&str, &str, Vec<&str>)>), E> {
    let (i, line) = line_parser_choices_default(i)?;
    let details = vec![];
    Ok((i, (line, details)))
}

pub(crate) fn line_parser_choices_all<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (Vec<&'a str>, Vec<(&'a str, &'a str, Vec<&'a str>)>), E> {
    let mut alternatives = alt((
        complete(line_parser_choices_all_locales),
        complete(line_parser_choices_all_default_only),
    ));
    let (i, (lines, locales)) = alternatives(i)?;
    Ok((i, (lines, locales)))
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::templates;
    use nom::error::VerboseError;

    #[test]
    fn test_line_parser_choices_locales_all() {
        let line = templates::getlines(&templates::apt_listchanges(), 4, 9999);
        match line_parser_choices_locales_all::<VerboseError<&str>>(&line) {
            Ok((_i, choices)) => {
                println!("choices {:?}", choices);
                println!("choices len {:?}", choices.len());
                assert!(choices.len() == 23);
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_choices_all() {
        let line = templates::getlines(&templates::apt_listchanges(), 3, 26);
        match line_parser_choices_all::<VerboseError<&str>>(&line) {
            Ok((_i, (choices, locales))) => {
                println!("choices {:?}", choices);
                println!("choices len {:?}", choices.len());
                println!("locales len {:?}", locales.len());
                assert!(choices.len() == 8);
                assert!(locales.len() == 23);
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_test_line_parser_choices_locales() {
        let line = templates::getlines(&templates::apt_listchanges(), 4, 4);
        match line_parser_choices_locales::<VerboseError<&str>>(&line) {
            Ok((_i, (country, encoding, choices))) => {
                println!("choices {:?}", choices);
                assert!(
                    choices
                        == vec![
                            "paginador",
                            "navegador",
                            "paginador-xterm",
                            "navegador-xterm",
                            "gtk",
                            "text",
                            "correu",
                            "cap"
                        ]
                );
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_test_line_parser_choices_default() {
        let line = templates::getlines(&templates::apt_listchanges(), 3, 3);
        match line_parser_choices_default::<VerboseError<&str>>(&line) {
            Ok((_i, choices)) => {
                println!("choices {:?}", choices);
                assert!(
                    choices
                        == vec![
                            "pager",
                            "browser",
                            "xterm-pager",
                            "xterm-browser",
                            "gtk",
                            "text",
                            "mail",
                            "none"
                        ]
                );
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_choices_default_variable() {
        let line = templates::getlines(&templates::ca_certificates(), 235, 236);
        match line_parser_choices_all::<VerboseError<&str>>(&line) {
            Ok((i, choices)) => {
                println!("choices {:?}", choices);
                println!("i {:?}", i);
                /* assert!(                     choices                         == vec![
                            "pager",
                            "browser",
                            "xterm-pager",
                            "xterm-browser",
                            "gtk",
                            "text",
                            "mail",
                            "none"
                        ]
                );*/
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_debconf_s() {
        let line = templates::getlines(&templates::debconf(), 2, 3);
        match line_parser_choices_all::<VerboseError<&str>>(&line) {
            Ok((i, choices)) => {
                println!("choices {:?}", choices);
                println!("i {:?}", i);
                assert!(i.len() == 0);
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
}
