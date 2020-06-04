use nom::bytes::complete::tag;
use nom::IResult;

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
