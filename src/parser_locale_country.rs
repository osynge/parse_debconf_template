use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::complete;
use nom::IResult;

fn locale_country_am(i: &str) -> IResult<&str, &str> {
    tag("am")(i)
}

fn locale_country_ar(i: &str) -> IResult<&str, &str> {
    tag("ar")(i)
}

fn locale_country_ast(i: &str) -> IResult<&str, &str> {
    tag("ast")(i)
}

fn locale_country_a(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_am),
        complete(locale_country_ar),
        complete(locale_country_ast),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_be(i: &str) -> IResult<&str, &str> {
    tag("be")(i)
}

fn locale_country_bg(i: &str) -> IResult<&str, &str> {
    tag("bg")(i)
}

fn locale_country_bn(i: &str) -> IResult<&str, &str> {
    tag("bn")(i)
}
fn locale_country_bs(i: &str) -> IResult<&str, &str> {
    tag("bs")(i)
}

fn locale_country_b(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_be),
        complete(locale_country_bg),
        complete(locale_country_bn),
        complete(locale_country_bs),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ca(i: &str) -> IResult<&str, &str> {
    tag("ca")(i)
}

fn locale_country_cs(i: &str) -> IResult<&str, &str> {
    tag("cs")(i)
}
fn locale_country_cy(i: &str) -> IResult<&str, &str> {
    tag("cy")(i)
}

fn locale_country_c(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_ca),
        complete(locale_country_cs),
        complete(locale_country_cy),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_da(i: &str) -> IResult<&str, &str> {
    tag("da")(i)
}

fn locale_country_de(i: &str) -> IResult<&str, &str> {
    tag("de")(i)
}
fn locale_country_dz(i: &str) -> IResult<&str, &str> {
    tag("dz")(i)
}

fn locale_country_d(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_da),
        complete(locale_country_de),
        complete(locale_country_dz),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_el(i: &str) -> IResult<&str, &str> {
    tag("el")(i)
}
fn locale_country_eo(i: &str) -> IResult<&str, &str> {
    tag("eo")(i)
}

fn locale_country_es(i: &str) -> IResult<&str, &str> {
    tag("es")(i)
}

fn locale_country_et(i: &str) -> IResult<&str, &str> {
    tag("et")(i)
}
fn locale_country_eu(i: &str) -> IResult<&str, &str> {
    tag("eu")(i)
}

fn locale_country_e(i: &str) -> IResult<&str, &str> {
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

fn locale_country_fa(i: &str) -> IResult<&str, &str> {
    tag("fa")(i)
}
fn locale_country_fi(i: &str) -> IResult<&str, &str> {
    tag("fi")(i)
}

fn locale_country_fr(i: &str) -> IResult<&str, &str> {
    tag("fr")(i)
}

fn locale_country_f(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_fa),
        complete(locale_country_fi),
        complete(locale_country_fr),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ga(i: &str) -> IResult<&str, &str> {
    tag("ga")(i)
}

fn locale_country_gl(i: &str) -> IResult<&str, &str> {
    tag("gl")(i)
}

fn locale_country_gu(i: &str) -> IResult<&str, &str> {
    tag("gu")(i)
}

fn locale_country_g(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_ga),
        complete(locale_country_gl),
        complete(locale_country_gu),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_he(i: &str) -> IResult<&str, &str> {
    tag("he")(i)
}
fn locale_country_hi(i: &str) -> IResult<&str, &str> {
    tag("hi")(i)
}

fn locale_country_hr(i: &str) -> IResult<&str, &str> {
    tag("hr")(i)
}

fn locale_country_hu(i: &str) -> IResult<&str, &str> {
    tag("hu")(i)
}

fn locale_country_h(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_he),
        complete(locale_country_hi),
        complete(locale_country_hr),
        complete(locale_country_hu),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}
fn locale_country_id(i: &str) -> IResult<&str, &str> {
    tag("id")(i)
}

fn locale_country_is(i: &str) -> IResult<&str, &str> {
    tag("is")(i)
}
fn locale_country_it(i: &str) -> IResult<&str, &str> {
    tag("it")(i)
}

fn locale_country_i(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_id),
        complete(locale_country_is),
        complete(locale_country_it),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ja(i: &str) -> IResult<&str, &str> {
    tag("ja")(i)
}

fn locale_country_ka(i: &str) -> IResult<&str, &str> {
    tag("ka")(i)
}
fn locale_country_kk(i: &str) -> IResult<&str, &str> {
    tag("kk")(i)
}

fn locale_country_km(i: &str) -> IResult<&str, &str> {
    tag("km")(i)
}

fn locale_country_kn(i: &str) -> IResult<&str, &str> {
    tag("kn")(i)
}

fn locale_country_ko(i: &str) -> IResult<&str, &str> {
    tag("ko")(i)
}

fn locale_country_ku(i: &str) -> IResult<&str, &str> {
    tag("ku")(i)
}

fn locale_country_k(i: &str) -> IResult<&str, &str> {
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

fn locale_country_lo(i: &str) -> IResult<&str, &str> {
    tag("lo")(i)
}

fn locale_country_lt(i: &str) -> IResult<&str, &str> {
    tag("lt")(i)
}

fn locale_country_lv(i: &str) -> IResult<&str, &str> {
    tag("lv")(i)
}

fn locale_country_l(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_lo),
        complete(locale_country_lt),
        complete(locale_country_lv),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_mk(i: &str) -> IResult<&str, &str> {
    tag("mk")(i)
}

fn locale_country_ml(i: &str) -> IResult<&str, &str> {
    tag("ml")(i)
}

fn locale_country_mr(i: &str) -> IResult<&str, &str> {
    tag("mr")(i)
}

fn locale_country_m(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_mk),
        complete(locale_country_ml),
        complete(locale_country_mr),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_nb(i: &str) -> IResult<&str, &str> {
    tag("nb")(i)
}

fn locale_country_ne(i: &str) -> IResult<&str, &str> {
    tag("ne")(i)
}

fn locale_country_nl(i: &str) -> IResult<&str, &str> {
    tag("nl")(i)
}

fn locale_country_nn(i: &str) -> IResult<&str, &str> {
    tag("nn")(i)
}

fn locale_country_no(i: &str) -> IResult<&str, &str> {
    tag("no")(i)
}

fn locale_country_n(i: &str) -> IResult<&str, &str> {
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

fn locale_country_pa(i: &str) -> IResult<&str, &str> {
    tag("pa")(i)
}

fn locale_country_pl(i: &str) -> IResult<&str, &str> {
    tag("pl")(i)
}

fn locale_country_pt_br(i: &str) -> IResult<&str, &str> {
    tag("pt_BR")(i)
}

fn locale_country_pt(i: &str) -> IResult<&str, &str> {
    tag("pt")(i)
}

fn locale_country_p(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_pa),
        complete(locale_country_pl),
        complete(locale_country_pt_br),
        complete(locale_country_pt),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ro(i: &str) -> IResult<&str, &str> {
    tag("ro")(i)
}

fn locale_country_ru(i: &str) -> IResult<&str, &str> {
    tag("ru")(i)
}

fn locale_country_r(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((complete(locale_country_ro), complete(locale_country_ru)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_si(i: &str) -> IResult<&str, &str> {
    tag("si")(i)
}

fn locale_country_sk(i: &str) -> IResult<&str, &str> {
    tag("sk")(i)
}

fn locale_country_sl(i: &str) -> IResult<&str, &str> {
    tag("sl")(i)
}

fn locale_country_sq(i: &str) -> IResult<&str, &str> {
    tag("sq")(i)
}

fn locale_country_sr(i: &str) -> IResult<&str, &str> {
    tag("sr")(i)
}

fn locale_country_sr_latin(i: &str) -> IResult<&str, &str> {
    tag("sr@latin")(i)
}

fn locale_country_sv(i: &str) -> IResult<&str, &str> {
    tag("sv")(i)
}

fn locale_country_s(i: &str) -> IResult<&str, &str> {
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

fn locale_country_ta(i: &str) -> IResult<&str, &str> {
    tag("ta")(i)
}
fn locale_country_te(i: &str) -> IResult<&str, &str> {
    tag("te")(i)
}
fn locale_country_tg(i: &str) -> IResult<&str, &str> {
    tag("tg")(i)
}
fn locale_country_th(i: &str) -> IResult<&str, &str> {
    tag("th")(i)
}
fn locale_country_tr(i: &str) -> IResult<&str, &str> {
    tag("tr")(i)
}

fn locale_country_t(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_ta),
        complete(locale_country_te),
        complete(locale_country_tg),
        complete(locale_country_th),
        complete(locale_country_tr),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_ug(i: &str) -> IResult<&str, &str> {
    tag("ug")(i)
}
fn locale_country_uk(i: &str) -> IResult<&str, &str> {
    tag("uk")(i)
}

fn locale_country_u(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((complete(locale_country_ug), complete(locale_country_uk)));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

fn locale_country_vi(i: &str) -> IResult<&str, &str> {
    tag("vi")(i)
}
fn locale_country_zh_cn(i: &str) -> IResult<&str, &str> {
    tag("zh_CN")(i)
}

fn locale_country_zh_tw(i: &str) -> IResult<&str, &str> {
    tag("zh_TW")(i)
}

fn locale_country_z(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_zh_cn),
        complete(locale_country_zh_tw),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn locale_country_a2g(i: &str) -> IResult<&str, &str> {
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

pub(super) fn locale_country_h2n(i: &str) -> IResult<&str, &str> {
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

pub(super) fn locale_country_m2z(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_p),
        complete(locale_country_r),
        complete(locale_country_s),
        complete(locale_country_t),
        complete(locale_country_u),
        complete(locale_country_vi),
        complete(locale_country_z),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}

pub(super) fn locale_country(i: &str) -> IResult<&str, &str> {
    let mut alternatives = alt((
        complete(locale_country_a2g),
        complete(locale_country_h2n),
        complete(locale_country_m2z),
    ));
    let (i, line) = alternatives(i)?;
    Ok((i, line))
}
