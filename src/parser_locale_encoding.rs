use nom::bytes::complete::tag;
use nom::combinator::complete;
use nom::IResult;

pub(crate) fn locale_encoding_utf8(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("UTF-8")(i)
}

pub(crate) fn locale_encoding(i: &str) -> IResult<&str, &str> {
    let mut alternatives = complete(locale_encoding_utf8);
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}
