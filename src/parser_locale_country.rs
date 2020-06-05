use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::complete;
use nom::error::ParseError;
use nom::IResult;

fn locale_country_am<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("am")(i)
}

fn locale_country_ar<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ar")(i)
}

fn locale_country_ast<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ast")(i)
}

fn locale_country_a<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_am),
        complete(locale_country_ar),
        complete(locale_country_ast),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_be<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("be")(i)
}

fn locale_country_bg<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("bg")(i)
}

fn locale_country_bn<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("bn")(i)
}
fn locale_country_bs<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("bs")(i)
}

fn locale_country_b<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_be),
        complete(locale_country_bg),
        complete(locale_country_bn),
        complete(locale_country_bs),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ca<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ca")(i)
}

fn locale_country_cs<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("cs")(i)
}
fn locale_country_cy<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("cy")(i)
}

fn locale_country_c<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_ca),
        complete(locale_country_cs),
        complete(locale_country_cy),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_da<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("da")(i)
}

fn locale_country_de<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("de")(i)
}
fn locale_country_dz<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("dz")(i)
}

fn locale_country_d<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_da),
        complete(locale_country_de),
        complete(locale_country_dz),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_el<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("el")(i)
}
fn locale_country_eo<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("eo")(i)
}

fn locale_country_es<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("es")(i)
}

fn locale_country_et<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("et")(i)
}
fn locale_country_eu<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("eu")(i)
}

fn locale_country_e<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_el),
        complete(locale_country_eo),
        complete(locale_country_es),
        complete(locale_country_et),
        complete(locale_country_eu),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_fa<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("fa")(i)
}
fn locale_country_fi<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("fi")(i)
}

fn locale_country_fr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("fr")(i)
}

fn locale_country_f<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_fa),
        complete(locale_country_fi),
        complete(locale_country_fr),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ga<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ga")(i)
}

fn locale_country_gl<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("gl")(i)
}

fn locale_country_gu<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("gu")(i)
}

fn locale_country_g<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_ga),
        complete(locale_country_gl),
        complete(locale_country_gu),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_he<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("he")(i)
}
fn locale_country_hi<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("hi")(i)
}

fn locale_country_hr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("hr")(i)
}

fn locale_country_hu<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("hu")(i)
}

fn locale_country_h<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_he),
        complete(locale_country_hi),
        complete(locale_country_hr),
        complete(locale_country_hu),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}
fn locale_country_id<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("id")(i)
}

fn locale_country_is<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("is")(i)
}
fn locale_country_it<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("it")(i)
}

fn locale_country_i<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_id),
        complete(locale_country_is),
        complete(locale_country_it),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ja<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ja")(i)
}

fn locale_country_ka<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ka")(i)
}
fn locale_country_kk<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("kk")(i)
}

fn locale_country_km<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("km")(i)
}

fn locale_country_kn<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("kn")(i)
}

fn locale_country_ko<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ko")(i)
}

fn locale_country_ku<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ku")(i)
}

fn locale_country_k<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_ka),
        complete(locale_country_kk),
        complete(locale_country_km),
        complete(locale_country_kn),
        complete(locale_country_ko),
        complete(locale_country_ku),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_lo<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("lo")(i)
}

fn locale_country_lt<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("lt")(i)
}

fn locale_country_lv<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("lv")(i)
}

fn locale_country_l<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_lo),
        complete(locale_country_lt),
        complete(locale_country_lv),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_mk<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("mk")(i)
}

fn locale_country_ml<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ml")(i)
}

fn locale_country_mr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("mr")(i)
}

fn locale_country_m<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_mk),
        complete(locale_country_ml),
        complete(locale_country_mr),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_nb<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("nb")(i)
}

fn locale_country_ne<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ne")(i)
}

fn locale_country_nl<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("nl")(i)
}

fn locale_country_nn<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("nn")(i)
}

fn locale_country_no<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("no")(i)
}

fn locale_country_n<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_nb),
        complete(locale_country_ne),
        complete(locale_country_nl),
        complete(locale_country_nn),
        complete(locale_country_no),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_mg<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("mg")(i)
}

fn locale_country_pa<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("pa")(i)
}

fn locale_country_pl<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("pl")(i)
}

fn locale_country_pt_br<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("pt_BR")(i)
}

fn locale_country_pt<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("pt")(i)
}

fn locale_country_p<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_pa),
        complete(locale_country_pl),
        complete(locale_country_pt_br),
        complete(locale_country_pt),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ro<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ro")(i)
}

fn locale_country_ru<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ru")(i)
}

fn locale_country_r<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((complete(locale_country_ro), complete(locale_country_ru)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_si<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("si")(i)
}

fn locale_country_sk<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("sk")(i)
}

fn locale_country_sl<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("sl")(i)
}

fn locale_country_sq<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("sq")(i)
}

fn locale_country_sr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("sr")(i)
}

fn locale_country_sr_latin<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("sr@latin")(i)
}

fn locale_country_sv<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("sv")(i)
}

fn locale_country_s<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_si),
        complete(locale_country_sk),
        complete(locale_country_sl),
        complete(locale_country_sq),
        complete(locale_country_sr_latin),
        complete(locale_country_sr),
        complete(locale_country_sv),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ta<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ta")(i)
}
fn locale_country_te<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("te")(i)
}
fn locale_country_tg<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("tg")(i)
}
fn locale_country_th<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("th")(i)
}
fn locale_country_tl<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("tl")(i)
}
fn locale_country_tr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("tr")(i)
}

fn locale_country_t<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_ta),
        complete(locale_country_te),
        complete(locale_country_tg),
        complete(locale_country_th),
        complete(locale_country_tl),
        complete(locale_country_tr),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ug<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("ug")(i)
}
fn locale_country_uk<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("uk")(i)
}

fn locale_country_u<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((complete(locale_country_ug), complete(locale_country_uk)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_wo<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("wo")(i)
}

fn locale_country_vi<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("vi")(i)
}

fn locale_country_zh_cn<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("zh_CN")(i)
}

fn locale_country_zh_tw<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("zh_TW")(i)
}

fn locale_country_z<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_zh_cn),
        complete(locale_country_zh_tw),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn locale_country_a2g<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_a),
        complete(locale_country_b),
        complete(locale_country_c),
        complete(locale_country_d),
        complete(locale_country_e),
        complete(locale_country_f),
        complete(locale_country_g),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn locale_country_h2n<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_h),
        complete(locale_country_i),
        complete(locale_country_ja),
        complete(locale_country_k),
        complete(locale_country_l),
        complete(locale_country_m),
        complete(locale_country_n),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn locale_country_m2z<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_mg),
        complete(locale_country_p),
        complete(locale_country_r),
        complete(locale_country_s),
        complete(locale_country_t),
        complete(locale_country_u),
        complete(locale_country_vi),
        complete(locale_country_wo),
        complete(locale_country_z),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn locale_country<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let mut alternatives = alt((
        complete(locale_country_a2g),
        complete(locale_country_h2n),
        complete(locale_country_m2z),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}
