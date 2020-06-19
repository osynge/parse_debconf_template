use crate::parser_choices::line_parser_choices_all;
use crate::parser_delimiters::{
    delimiter_description_locale, delimiter_key_value, delimiter_line,
    delimiter_locale_country_encoding, delimiter_package_section,
};
use crate::parser_description;
use crate::parser_description::line_parser_decription_sections_all;
use crate::parser_locale_country::locale_country;
use crate::parser_locale_encoding::locale_encoding;
use core::option;
use indoc::indoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::complete;
use nom::combinator::peek;
use nom::error::ParseError;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::string::String;

pub(crate) fn key_template<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("Template")(i)
}

pub(crate) fn key_type<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("Type")(i)
}

pub(crate) fn key_default<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("Default")(i)
}

pub(crate) fn template_type_boolean<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("boolean")(i)
}

pub(crate) fn template_type_error<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("error")(i)
}

pub(crate) fn template_type_multiselect<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("multiselect")(i)
}

pub(crate) fn template_type_text<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("text")(i)
}

pub(crate) fn template_type_select<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("select")(i)
}

pub(crate) fn template_type_string<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("string")(i)
}

pub(crate) fn template_type_title<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("title")(i)
}

pub(crate) fn template_type<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(template_type_boolean),
        complete(template_type_error),
        complete(template_type_select),
        complete(template_type_string),
        complete(template_type_multiselect),
        complete(template_type_text),
        complete(template_type_title),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn line_parser_template<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str), E> {
    let (i, _) = key_template(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, package) = nom::bytes::complete::is_not("/")(i)?;
    let (i, _) = delimiter_package_section(i)?;
    let (i, section) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, (package, section)))
}

fn line_parser_type<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let (i, _) = key_type(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, template_type) = template_type(i)?;
    Ok((i, template_type))
}

fn line_parser_default<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let (i, _) = key_default(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, default) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, default))
}

fn section_parser<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<
    &'a str,
    (
        &'a str,
        &'a str,
        &'a str,
        Option<(Vec<&'a str>, Vec<(&'a str, &'a str, Vec<&'a str>)>)>,
        Option<&'a str>,
        &'a str,
        Vec<&'a str>,
        Vec<(&'a str, &'a str, &'a str, Vec<&'a str>)>,
    ),
    E,
> {
    let (i, (package, section)) = line_parser_template(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, template_type) = line_parser_type(i)?;
    let (i, _) = delimiter_line(i)?;
    // std::result::Result<(&str, &str), nom::Err<E>>

    let res: Result<(&str, &str), nom::Err<E>> = tag("Choices")(i);
    let (i, opt_choices) = match res {
        Ok(_) => {
            let (i, choices) = line_parser_choices_all(i)?;
            let (i, _) = delimiter_line(i)?;
            (i, Some(choices))
        }
        Err(_) => (i, None),
    };
    let res: Result<(&str, &str), nom::Err<E>> = tag("Default")(i);
    let (i, opt_default) = match res {
        Ok(_) => {
            let (i, default) = line_parser_default(i)?;
            let (i, _) = delimiter_line(i)?;
            (i, Some(default))
        }
        Err(_) => (i, None),
    };
    let (i, (decription_title, decription_lines, decription_locales)) =
        line_parser_decription_sections_all(i)?;
    Ok((
        i,
        (
            package,
            section,
            template_type,
            opt_choices,
            opt_default,
            decription_title,
            decription_lines,
            decription_locales,
        ),
    ))
}

pub(super) fn template_parser<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<
    &'a str,
    Vec<(
        &'a str,
        &'a str,
        &'a str,
        Option<(Vec<&'a str>, Vec<(&'a str, &'a str, Vec<&'a str>)>)>,
        Option<&'a str>,
        &'a str,
        Vec<&'a str>,
        Vec<(&'a str, &'a str, &'a str, Vec<&'a str>)>,
    )>,
    E,
> {
    separated_list1(tag("\n\n"), section_parser)(i)
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::templates;
    use nom::error::VerboseError;

    #[test]
    fn test_line_parser_template_dash() {
        let line = templates::getlines(&templates::dash(), 0, 0);
        match line_parser_template::<VerboseError<&str>>(&line) {
            Ok((i, (package, section))) => {
                println!("package {:?}", package);

                println!("package {:?}", section);
                assert!(package == "dash");
                assert!(section == "sh");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_type() {
        let line = templates::getlines(&templates::dash(), 1, 1);
        match line_parser_type::<VerboseError<&str>>(&line) {
            Ok((i, template_type)) => {
                println!("template_type {:?}", template_type);
                assert!(template_type == "boolean");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_default() {
        let line = templates::getlines(&templates::dash(), 2, 2);
        match line_parser_default::<VerboseError<&str>>(&line) {
            Ok((i, default)) => {
                println!("default {:?}", default);
                assert!(default == "true");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_section_parser_choice_defaulted() {
        let line = String::from(templates::getlines(&templates::apt_listchanges(), 1, 72));
        //println!("line {:?}", line);

        match section_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i.len());
                assert!(i == "");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_section_parser_choice_nodefault() {
        let line = String::from(templates::getlines(
            &templates::ca_certificates(),
            233,
            9999,
        ));
        //println!("line {:?}", line);

        match section_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i.len());
                assert!(i == "");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_sections_all() {
        let line = String::from(templates::getlines(&templates::dash(), 3, 9999));
        match line_parser_decription_sections_all::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("i {:?}", i);
                println!("value {:?}", value);
                assert!(i == "");
                /*assert!(value == ("bg", "UTF-8","Използване на dash като системна обвивка (/bin/sh)?", vec!["Системната обвивка се използва по подразбиране от скриптовете на обвивката.", "", "Използването на dash като системна обвивка ще подобри бързодействието на системата като цяло. Тази настройка не променя обвивката на интерактивните потребители."]));*/
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_dash_section_parser() {
        let line = String::from(templates::getlines(&templates::dash(), 0, 9999));
        match section_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }
}
