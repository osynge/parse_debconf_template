use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::complete;
use nom::IResult;

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
    let mut alternatives = alt((
        complete(locale_country_nb),
        complete(locale_country_nl),
        complete(locale_country_no),
    ));
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

pub(super) fn locale_country(i: &str) -> IResult<&str, &str> {
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
