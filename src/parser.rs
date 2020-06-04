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
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::string::String;

pub(crate) fn key_template(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("Template")(i)
}

pub(crate) fn key_type(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("Type")(i)
}

pub(crate) fn key_default(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("Default")(i)
}

pub(crate) fn template_type_boolean(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("boolean")(i)
}

pub(crate) fn template_type_error(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("error")(i)
}

pub(crate) fn template_type_multiselect(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("multiselect")(i)
}

pub(crate) fn template_type_text(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("text")(i)
}

pub(crate) fn template_type_select(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("select")(i)
}

pub(crate) fn template_type_string(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("string")(i)
}

pub(crate) fn template_type_title(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("title")(i)
}

pub(crate) fn template_type(i: &str) -> IResult<&str, &str> {
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

fn line_parser_template(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, _) = key_template(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, package) = nom::bytes::complete::is_not("/")(i)?;
    let (i, _) = delimiter_package_section(i)?;
    let (i, section) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, (package, section)))
}

fn line_parser_type(i: &str) -> IResult<&str, &str> {
    let (i, _) = key_type(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, template_type) = template_type(i)?;
    Ok((i, template_type))
}

fn line_parser_default(i: &str) -> IResult<&str, &str> {
    let (i, _) = key_default(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, default) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, default))
}

fn section_parser_choice_defaulted(
    i: &str,
) -> IResult<
    &str,
    (
        &str,
        &str,
        &str,
        Option<(Vec<&str>, Vec<(&str, &str, Vec<&str>)>)>,
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    ),
> {
    let (i, (package, section)) = line_parser_template(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (template_type)) = line_parser_type(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, choices) = line_parser_choices_all(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, template_default) = line_parser_default(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (decription_title, decription_lines, decription_locales)) =
        line_parser_decription_sections_all(i)?;
    Ok((
        i,
        (
            package,
            section,
            template_type,
            Some(choices),
            Some(template_default),
            decription_title,
            decription_lines,
            decription_locales,
        ),
    ))
}

fn section_parser_choice_nodefault(
    i: &str,
) -> IResult<
    &str,
    (
        &str,
        &str,
        &str,
        Option<(Vec<&str>, Vec<(&str, &str, Vec<&str>)>)>,
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    ),
> {
    let (i, (package, section)) = line_parser_template(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (template_type)) = line_parser_type(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, choices) = line_parser_choices_all(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (decription_title, decription_lines, decription_locales)) =
        line_parser_decription_sections_all(i)?;
    Ok((
        i,
        (
            package,
            section,
            template_type,
            Some(choices),
            None,
            decription_title,
            decription_lines,
            decription_locales,
        ),
    ))
}

fn section_parser_defaulted(
    i: &str,
) -> IResult<
    &str,
    (
        &str,
        &str,
        &str,
        Option<(Vec<&str>, Vec<(&str, &str, Vec<&str>)>)>,
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    ),
> {
    let (i, (package, section)) = line_parser_template(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (template_type)) = line_parser_type(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, template_default) = line_parser_default(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (decription_title, decription_lines, decription_locales)) =
        line_parser_decription_sections_all(i)?;
    Ok((
        i,
        (
            package,
            section,
            template_type,
            None,
            Some(template_default),
            decription_title,
            decription_lines,
            decription_locales,
        ),
    ))
}

fn section_parser_nodefault(
    i: &str,
) -> IResult<
    &str,
    (
        &str,
        &str,
        &str,
        Option<(Vec<&str>, Vec<(&str, &str, Vec<&str>)>)>,
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    ),
> {
    let (i, (package, section)) = line_parser_template(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (template_type)) = line_parser_type(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (decription_title, decription_lines, decription_locales)) =
        line_parser_decription_sections_all(i)?;
    Ok((
        i,
        (
            package,
            section,
            template_type,
            None,
            None,
            decription_title,
            decription_lines,
            decription_locales,
        ),
    ))
}

fn section_parser(
    i: &str,
) -> IResult<
    &str,
    (
        &str,
        &str,
        &str,
        Option<(Vec<&str>, Vec<(&str, &str, Vec<&str>)>)>,
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    ),
> {
    let mut alternatives = alt((
        complete(section_parser_choice_defaulted),
        complete(section_parser_choice_nodefault),
        complete(section_parser_defaulted),
        complete(section_parser_nodefault),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn template_parser(
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
    separated_list1(tag("\n\n"), section_parser)(i)
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::templates;
    #[test]
    fn test_line_parser_template_dash() {
        let line = templates::getlines(&templates::dash(), 0, 0);
        match line_parser_template(&line) {
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
        match line_parser_type(&line) {
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
        match line_parser_default(&line) {
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
        let mut line = String::from(templates::getlines(&templates::apt_listchanges(), 1, 72));
        //println!("line {:?}", line);

        match section_parser_choice_defaulted(&line) {
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
        let mut line = String::from(templates::getlines(
            &templates::ca_certificates(),
            233,
            9999,
        ));
        //println!("line {:?}", line);

        match section_parser_choice_nodefault(&line) {
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
        let mut line = String::from(templates::getlines(&templates::dash(), 3, 9999));
        match line_parser_decription_sections_all(&line) {
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
        let mut line = String::from(templates::getlines(&templates::dash(), 0, 9999));
        match section_parser(&line) {
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
