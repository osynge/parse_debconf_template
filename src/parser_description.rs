use crate::parser_delimiters::{
    delimiter_description_locale, delimiter_key_value, delimiter_line,
    delimiter_locale_country_encoding,
};
use crate::parser_locale_country::locale_country;
use crate::parser_locale_encoding::locale_encoding;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::complete;
use nom::combinator::peek;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn key_description<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("Description")(i)
}

pub(crate) fn key_description_cont_blank<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag(" .")(i)
}

pub(crate) fn key_description_cont<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag(" ")(i)
}

fn line_parser_decription_title_has<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let (i, _) = key_description(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, title) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, title))
}
fn line_parser_decription_title_none<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let (i, _) = key_description(i)?;
    let (i, _) = nom::bytes::complete::tag(":")(i)?;
    Ok((i, ""))
}

fn line_parser_decription_title<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(line_parser_decription_title_has),
        complete(line_parser_decription_title_none),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn line_parser_decription_title_locales<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str, &'a str), E> {
    let (i, _) = key_description(i)?;
    let (i, _) = delimiter_description_locale(i)?;
    let (i, country) = locale_country(i)?;
    let (i, _) = delimiter_locale_country_encoding(i)?;
    let (i, encoding) = locale_encoding(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, title) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, (country, encoding, title)))
}

fn line_parser_decription_line<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let (i, _) = key_description_cont(i)?;
    let (i, line) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, line))
}

fn line_parser_decription_line_blank<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let (i, _) = key_description_cont_blank(i)?;
    let (i, _) = peek(tag("\n"))(i)?;

    Ok((i, ""))
}

fn line_parser_decription_lines<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(line_parser_decription_line_blank),
        complete(line_parser_decription_line),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn line_parser_decription_lines_full<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut tpl = tuple((line_parser_decription_lines, delimiter_line));
    let (i, (line, _)) = tpl(i)?;
    Ok((i, line))
}

fn line_parser_decription_lines_many<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> {
    separated_list1(tag("\n"), line_parser_decription_lines)(i)
}

fn line_parser_decription_section_lines<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, Vec<&'a str>), E> {
    let mut tpl = tuple((
        line_parser_decription_title,
        delimiter_line,
        line_parser_decription_lines_many,
    ));
    let (i, (line, _, details)) = tpl(i)?;

    Ok((i, (line, details)))
}

fn line_parser_decription_section_short<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, Vec<&'a str>), E> {
    let (i, line) = line_parser_decription_title(i)?;
    let details = vec![];
    Ok((i, (line, details)))
}

fn line_parser_decription_section<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, Vec<&'a str>), E> {
    let mut alternatives = alt((
        complete(line_parser_decription_section_lines),
        complete(line_parser_decription_section_short),
    ));
    let (i, (line, details)) = alternatives(i)?;
    Ok((i, (line, details)))
}

fn line_parser_decription_section_locales_lines<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str, &'a str, Vec<&'a str>), E> {
    let mut tpl = tuple((
        line_parser_decription_title_locales,
        delimiter_line,
        line_parser_decription_lines_many,
    ));
    let (i, ((country, encoding, title), _, details)) = tpl(i)?;
    Ok((i, (country, encoding, title, details)))
}

fn line_parser_decription_section_locales_short<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str, &'a str, Vec<&'a str>), E> {
    let (i, (country, encoding, title)) = line_parser_decription_title_locales(i)?;
    let details = vec![];
    Ok((i, (country, encoding, title, details)))
}

fn line_parser_decription_section_locales<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str, &'a str, Vec<&'a str>), E> {
    let mut alternatives = alt((
        complete(line_parser_decription_section_locales_lines),
        complete(line_parser_decription_section_locales_short),
    ));
    let (i, (country, encoding, title, details)) = alternatives(i)?;
    Ok((i, (country, encoding, title, details)))
}

pub(crate) fn line_parser_decription_sections_all_locale<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<
    &'a str,
    (
        &'a str,
        Vec<&'a str>,
        Vec<(&'a str, &'a str, &'a str, Vec<&'a str>)>,
    ),
    E,
> {
    let (i, (title, lines)) = line_parser_decription_section(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, locales) = separated_list1(tag("\n"), line_parser_decription_section_locales)(i)?;
    Ok((i, (title, lines, locales)))
}

pub(crate) fn line_parser_decription_sections_all_short<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<
    &'a str,
    (
        &'a str,
        Vec<&'a str>,
        Vec<(&'a str, &'a str, &'a str, Vec<&'a str>)>,
    ),
    E,
> {
    let (i, (title, lines)) = line_parser_decription_section(i)?;
    let locales = vec![];
    Ok((i, (title, lines, locales)))
}

pub(crate) fn line_parser_decription_sections_all<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<
    &'a str,
    (
        &'a str,
        Vec<&'a str>,
        Vec<(&'a str, &'a str, &'a str, Vec<&'a str>)>,
    ),
    E,
> {
    let mut alternatives = alt((
        complete(line_parser_decription_sections_all_locale),
        complete(line_parser_decription_sections_all_short),
    ));
    let (i, (title, lines, locales)) = alternatives(i)?;
    Ok((i, (title, lines, locales)))
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::templates;
    use nom::error::VerboseError;

    #[test]
    fn test_line_parser_decription_lines_all_locales() {
        let mut line = String::from(templates::getlines(&templates::dash(), 9, 27));
        line.push('\n');
        match line_parser_decription_section_locales::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                assert!(value == ("bg", "UTF-8","Използване на dash като системна обвивка (/bin/sh)?", vec!["Системната обвивка се използва по подразбиране от скриптовете на обвивката.", "", "Използването на dash като системна обвивка ще подобри бързодействието на системата като цяло. Тази настройка не променя обвивката на интерактивните потребители."]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_title() {
        let line = templates::getlines(&templates::dash(), 3, 3);
        match line_parser_decription_title::<VerboseError<&str>>(&line) {
            Ok((_i, title)) => {
                println!("title {:?}", title);
                assert!(title == "Use dash as the default system shell (/bin/sh)?")
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_line_parser_decription_title_locales() {
        let line = templates::getlines(&templates::dash(), 9, 9);
        match line_parser_decription_title_locales::<VerboseError<&str>>(&line) {
            Ok((_i, title)) => {
                println!("title {:?}", title);
                assert!(
                    title
                        == (
                            "bg",
                            "UTF-8",
                            "Използване на dash като системна обвивка (/bin/sh)?"
                        )
                )
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_line() {
        let line = templates::getlines(&templates::dash(), 4, 4);
        match line_parser_decription_line::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                assert!(
                    value
                        == "The system shell is the default command interpreter for shell scripts."
                )
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_line_blank() {
        let line = templates::getlines(&templates::dash(), 5, 6);
        match line_parser_decription_line_blank::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                assert!(value == "");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_lines_with_data() {
        let line = templates::getlines(&templates::dash(), 4, 5);
        match line_parser_decription_lines::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                assert!(
                    value
                        == "The system shell is the default command interpreter for shell scripts."
                );
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_line_parser_decription_lines_without_data() {
        let line = templates::getlines(&templates::dash(), 5, 6);
        match line_parser_decription_lines::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                assert!(value == "");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_lines_many() {
        let mut line = String::from(templates::getlines(&templates::dash(), 4, 27));
        line.push('\n');
        match line_parser_decription_lines_many::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                assert!(value == ["The system shell is the default command interpreter for shell scripts.", "", "Using dash as the system shell will improve the system\'s overall", "performance. It does not alter the shell presented to interactive", "users."]);
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_lines_all() {
        let mut line = String::from(templates::getlines(&templates::dash(), 3, 27));
        line.push('\n');
        match line_parser_decription_section::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                assert!(value == ("Use dash as the default system shell (/bin/sh)?", vec!["The system shell is the default command interpreter for shell scripts.", "", "Using dash as the system shell will improve the system\'s overall", "performance. It does not alter the shell presented to interactive", "users."]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_line_parser_decription_section_short() {
        let line = templates::getlines(&templates::ca_certificates(), 3, 6);
        println!("line {:?}", line);
        match line_parser_decription_section_short::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                //                assert!(value == ("", vec![]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_section() {
        let line = templates::getlines(&templates::ca_certificates(), 3, 6);
        println!("line {:?}", line);
        match line_parser_decription_section::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                //                assert!(value == ("", vec![]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_line_parser_decription_section_locales() {
        let line = templates::getlines(&templates::ca_certificates(), 4, 6);
        println!("line {:?}", line);
        match line_parser_decription_section_locales::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                //                assert!(value == ("", vec![]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_short() {
        let line = templates::getlines(&templates::ca_certificates(), 3, 6);
        println!("line {:?}", line);
        match line_parser_decription_sections_all::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                println!("value {:?}", value);
                //                assert!(value == ("", vec![]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_wamerican_line_parser_decription_sections_all() {
        let line = String::from(templates::getlines(&templates::wamerican(), 2, 9999));
        println!("line {:?}", line);
        match line_parser_decription_sections_all::<VerboseError<&str>>(&line) {
            Ok((_i, value)) => {
                assert!(value == ("", vec![], vec![]));
                /*assert!(value == ("bg", "UTF-8","Използване на dash като системна обвивка (/bin/sh)?", vec!["Системната обвивка се използва по подразбиране от скриптовете на обвивката.", "", "Използването на dash като системна обвивка ще подобри бързодействието на системата като цяло. Тази настройка не променя обвивката на интерактивните потребители."]));*/
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
}
