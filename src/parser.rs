use core::option;
use indoc::indoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::complete;
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

pub(crate) fn key_description(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("Description")(i)
}

pub(crate) fn key_description_cont_blank(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag(" .")(i)
}

pub(crate) fn key_description_cont(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag(" ")(i)
}

pub(crate) fn delimiter_line(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("\n")(i)
}

pub(crate) fn delimiter_key_value(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag(": ")(i)
}

pub(crate) fn delimiter_package_section(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("/")(i)
}

pub(crate) fn delimiter_description_locale(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("-")(i)
}

pub(crate) fn delimiter_locale_country_encoding(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag(".")(i)
}

pub(crate) fn template_type_boolean(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("boolean")(i)
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

pub(crate) fn template_type(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(template_type_boolean),
        complete(template_type_select),
        complete(template_type_string),
        complete(template_type_text),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(crate) fn locale_country_bg(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("bg")(i)
}

pub(crate) fn locale_country_ca(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("ca")(i)
}

pub(crate) fn locale_country_cs(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("cs")(i)
}

pub(crate) fn locale_country_c(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((complete(locale_country_ca), complete(locale_country_cs)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(crate) fn locale_country_da(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("da")(i)
}

pub(crate) fn locale_country_de(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("de")(i)
}

pub(crate) fn locale_country_d(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((complete(locale_country_da), complete(locale_country_de)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(crate) fn locale_country_es(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("es")(i)
}

pub(crate) fn locale_country_eu(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("eu")(i)
}

pub(crate) fn locale_country_fi(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("fi")(i)
}

pub(crate) fn locale_country_fr(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("fr")(i)
}

pub(crate) fn locale_country_gl(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("gl")(i)
}

pub(crate) fn locale_country_id(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("id")(i)
}

pub(crate) fn locale_country_it(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("it")(i)
}

pub(crate) fn locale_country_ja(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("ja")(i)
}

pub(crate) fn locale_country_nb(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("nb")(i)
}

pub(crate) fn locale_country_nl(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("nl")(i)
}

pub(crate) fn locale_country_no(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("no")(i)
}


pub(crate) fn locale_country_n(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((complete(locale_country_nb), complete(locale_country_nl), complete(locale_country_no)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(crate) fn locale_country_pl(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("pl")(i)
}

pub(crate) fn locale_country_pt_BR(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("pt_BR")(i)
}

pub(crate) fn locale_country_pt(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("pt")(i)
}

pub(crate) fn locale_country_p(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_pl),
        complete(locale_country_pt_BR),
        complete(locale_country_pt),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(crate) fn locale_country_ro(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("ro")(i)
}

pub(crate) fn locale_country_ru(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("ru")(i)
}

pub(crate) fn locale_country_sk(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("sk")(i)
}

pub(crate) fn locale_country_sr(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("sr")(i)
}

pub(crate) fn locale_country_sv(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("sv")(i)
}

pub(crate) fn locale_country_tr(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("tr")(i)
}

pub(crate) fn locale_country_vi(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("vi")(i)
}
pub(crate) fn locale_country_zh_CN(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("zh_CN")(i)
}

pub(crate) fn locale_country_zh_TW(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("zh_TW")(i)
}



pub(crate) fn locale_country_z(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_zh_CN),
        complete(locale_country_zh_TW),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}


fn locale_country(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_bg),
        complete(locale_country_c),
        complete(locale_country_d),
        complete(locale_country_es),
        complete(locale_country_eu),
        complete(locale_country_fi),
        complete(locale_country_fr),
        complete(locale_country_gl),
        complete(locale_country_id),
        complete(locale_country_it),
        complete(locale_country_ja),
        complete(locale_country_n),
        complete(locale_country_p),
        complete(locale_country_ro),
        complete(locale_country_ru),
        complete(locale_country_sk),
        complete(locale_country_sr),
        complete(locale_country_sv),
        complete(locale_country_tr),
        complete(locale_country_vi),
        complete(locale_country_z),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(crate) fn locale_encoding_utf8(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("UTF-8")(i)
}

fn locale_encoding(i: &str) -> IResult<&str, &str> {
    let mut alternatives = complete(locale_encoding_utf8);
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

fn line_parser_decription_title_has(i: &str) -> IResult<&str, &str> {
    let (i, _) = key_description(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, title) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, title))
}
fn line_parser_decription_title_none(i: &str) -> IResult<&str, &str> {
    let (i, _) = key_description(i)?;
    let (i, _) = nom::bytes::complete::tag(":")(i)?;
    Ok((i, ""))
}

fn line_parser_decription_title(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(line_parser_decription_title_has),
        complete(line_parser_decription_title_none),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn line_parser_decription_title_locales(i: &str) -> IResult<&str, (&str, &str, &str)> {
    let (i, _) = key_description(i)?;
    let (i, _) = delimiter_description_locale(i)?;
    let (i, country) = locale_country(i)?;
    let (i, _) = delimiter_locale_country_encoding(i)?;
    let (i, encoding) = locale_encoding(i)?;
    let (i, _) = delimiter_key_value(i)?;
    let (i, title) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, (country, encoding, title)))
}

fn line_parser_decription_line(i: &str) -> IResult<&str, &str> {
    let (i, _) = key_description_cont(i)?;
    let (i, line) = nom::bytes::complete::is_not("\n")(i)?;
    Ok((i, line))
}

fn line_parser_decription_line_blank(i: &str) -> IResult<&str, &str> {
    let (i, _) = key_description_cont_blank(i)?;
    Ok((i, ""))
}

fn line_parser_decription_lines(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(line_parser_decription_line_blank),
        complete(line_parser_decription_line),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn line_parser_decription_lines_full(i: &str) -> IResult<&str, &str> {
    let mut tpl = tuple((line_parser_decription_lines, delimiter_line));
    let (i, (line, _)) = tpl(i)?;
    Ok((i, line))
}

fn line_parser_decription_lines_many(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag("\n"), line_parser_decription_lines)(i)
}

fn line_parser_decription_section(i: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let mut tpl = tuple((
        line_parser_decription_title,
        delimiter_line,
        line_parser_decription_lines_many,
    ));
    let (i, (line, _, details)) = tpl(i)?;

    Ok((i, (line, details)))
}

fn line_parser_decription_section_locales(i: &str) -> IResult<&str, (&str, &str, &str, Vec<&str>)> {
    let mut tpl = tuple((
        line_parser_decription_title_locales,
        delimiter_line,
        line_parser_decription_lines_many,
    ));
    let (i, ((country, encoding, title), _, details)) = tpl(i)?;

    Ok((i, (country, encoding, title, details)))
}

fn line_parser_decription_sections_all(
    i: &str,
) -> IResult<&str, (&str, Vec<&str>, Vec<(&str, &str, &str, Vec<&str>)>)> {
    let (i, (title, lines)) = line_parser_decription_section(i)?;
    let (i, _) = delimiter_line(i)?;
    let (i, (locales)) = separated_list0(tag("\n"), line_parser_decription_section_locales)(i)?;
    Ok((i, (title, lines, locales)))
}

fn section_parser_defaulted(
    i: &str,
) -> IResult<
    &str,
    (
        &str,
        &str,
        &str,
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
        Option<&str>,
        &str,
        Vec<&str>,
        Vec<(&str, &str, &str, Vec<&str>)>,
    ),
> {
    let mut alternatives = alt((
        complete(section_parser_defaulted),
        complete(section_parser_nodefault),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn template_parser(
    i: &str,
) -> IResult<
    &str,
    Vec<(
        &str,
        &str,
        &str,
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
    fn test_line_parser_decription_title() {
        let line = templates::getlines(&templates::dash(), 3, 3);
        match line_parser_decription_title(&line) {
            Ok((i, title)) => {
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
        match line_parser_decription_title_locales(&line) {
            Ok((i, title)) => {
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
        match line_parser_decription_line(&line) {
            Ok((i, value)) => {
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
        let line = templates::getlines(&templates::dash(), 5, 5);
        match line_parser_decription_line_blank(&line) {
            Ok((i, value)) => {
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
        match line_parser_decription_lines(&line) {
            Ok((i, value)) => {
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
        match line_parser_decription_lines(&line) {
            Ok((i, value)) => {
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
        match line_parser_decription_lines_many(&line) {
            Ok((i, value)) => {
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
        match line_parser_decription_section(&line) {
            Ok((i, value)) => {
                assert!(value == ("Use dash as the default system shell (/bin/sh)?", vec!["The system shell is the default command interpreter for shell scripts.", "", "Using dash as the system shell will improve the system\'s overall", "performance. It does not alter the shell presented to interactive", "users."]));
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_line_parser_decription_lines_all_locales() {
        let mut line = String::from(templates::getlines(&templates::dash(), 9, 27));
        line.push('\n');
        match line_parser_decription_section_locales(&line) {
            Ok((i, value)) => {
                assert!(value == ("bg", "UTF-8","Използване на dash като системна обвивка (/bin/sh)?", vec!["Системната обвивка се използва по подразбиране от скриптовете на обвивката.", "", "Използването на dash като системна обвивка ще подобри бързодействието на системата като цяло. Тази настройка не променя обвивката на интерактивните потребители."]));
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
    fn test_wamerican_line_parser_decription_sections_all() {
        let mut line = String::from(templates::getlines(&templates::wamerican(), 2, 9999));
        println!(" {:?}", line);
        match line_parser_decription_sections_all(&line) {
            Ok((i, value)) => {
                assert!(value == ("", vec![], vec![]));
                /*assert!(value == ("bg", "UTF-8","Използване на dash като системна обвивка (/bin/sh)?", vec!["Системната обвивка се използва по подразбиране от скриптовете на обвивката.", "", "Използването на dash като системна обвивка ще подобри бързодействието на системата като цяло. Тази настройка не променя обвивката на интерактивните потребители."]));*/
            }
            Err(err) => {
                println!("err {:?}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_dash_all() {
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
    #[test]
    fn test_adduser_all_template_parser() {
        let mut line = String::from(templates::getlines(&templates::adduser(), 0, 9999));
        match template_parser(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                // Test must fail as starts with blank line
                assert!(false);
            }
            Err(err) => {
                println!("err {:?}", err);
                //assert!(false);
            }
        }
    }

    #[test]
    fn test_apparmor_all_template_parser() {
        let mut line = String::from(templates::getlines(&templates::apparmor(), 0, 9999));
        match template_parser(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                // Test must fail as starts with blank line
                assert!(false);
            }
            Err(err) => {
                println!("err {:?}", err);
                //assert!(false);
            }
        }
    }

    #[test]
    fn test_irqbalance_all_template_parser() {
        let mut line = String::from(templates::getlines(&templates::irqbalance(), 0, 9999));
        match template_parser(&line) {
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
