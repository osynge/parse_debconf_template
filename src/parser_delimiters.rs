use nom::bytes::complete::tag;
use nom::error::ParseError;
use nom::IResult;

pub(crate) fn delimiter_line<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    tag("\n")(i)
}

pub(crate) fn delimiter_key_value<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    tag(": ")(i)
}

pub(crate) fn delimiter_package_section<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    tag("/")(i)
}

pub(crate) fn delimiter_description_locale<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    tag("-")(i)
}

pub(crate) fn delimiter_locale_country_encoding<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    tag(".")(i)
}
