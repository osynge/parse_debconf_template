use nom::bytes::complete::tag;
use nom::combinator::complete;
use nom::error::ParseError;
use nom::IResult;

pub(crate) fn locale_encoding_utf8<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::tag("UTF-8")(i)
}

pub(crate) fn locale_encoding<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = complete(locale_encoding_utf8);
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}
