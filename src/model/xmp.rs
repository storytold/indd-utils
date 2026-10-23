//! Dates of the document's XMP packet, which give the time zone offsets
//! that IDML link times are written in (`docs/format/objects.md`, link
//! times), and the name of its black swatch, which shows the language of
//! the edition that saved it (`docs/format/objects.md`, saving edition).

/// An XMP date with a UTC offset: the instant (seconds since 1970 UTC)
/// and the offset in seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XmpDate {
    pub utc: i64,
    pub offset: i64,
}

/// The properties whose dates are read.
const DATE_PROPERTIES: [&str; 4] = [
    "xmp:CreateDate",
    "xmp:MetadataDate",
    "xmp:ModifyDate",
    "stEvt:when",
];

/// The dates of `packet` that have a UTC offset (`Z` or `±HH:MM`), from
/// the properties `xmp:CreateDate`, `xmp:MetadataDate`, `xmp:ModifyDate`
/// and `stEvt:when` (each in attribute or element form). Dates without
/// an offset or that do not parse are left out.
pub fn dates(packet: &[u8]) -> Vec<XmpDate> {
    let text = String::from_utf8_lossy(packet);
    let mut out = Vec::new();
    for name in DATE_PROPERTIES {
        let mut rest = &text[..];
        while let Some(i) = rest.find(name) {
            rest = &rest[i + name.len()..];
            let value = if let Some(r) = rest.strip_prefix("=\"") {
                r
            } else if let Some(r) = rest.strip_prefix('>') {
                r
            } else {
                continue;
            };
            if let Some(d) = parse(value) {
                out.push(d);
            }
        }
    }
    out
}

/// A colour of the packet's `xmpTPg:Colorants` list.
#[derive(Debug, Clone, PartialEq)]
pub struct Colorant {
    pub name: String,
    /// `CMYK`, `RGB` or `LAB`.
    pub mode: String,
    /// `PROCESS` or `SPOT`, in either case.
    pub kind: String,
    /// The colorant is a tint (`xmpG:tint`).
    pub tint: bool,
    /// Cyan, magenta, yellow, black in percent, or red, green, blue
    /// (0–255), or L, a, b; as many as the mode has.
    pub values: Vec<f64>,
}

/// The value of property `name` in `item`, in element (`<name>v</name>`)
/// or attribute (`name="v"`) form, with the XML entities resolved.
fn property(item: &str, name: &str) -> Option<String> {
    let raw = if let Some(i) = item.find(&format!("<{name}>")) {
        let rest = &item[i + name.len() + 2..];
        &rest[..rest.find('<')?]
    } else {
        let i = item.find(&format!("{name}=\""))?;
        let rest = &item[i + name.len() + 2..];
        &rest[..rest.find('"')?]
    };
    Some(
        raw.replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&amp;", "&"),
    )
}

/// The colours of the `xmpTPg:Colorants` list of `packet`. Items without
/// a name or a mode are left out.
pub fn colorants(packet: &[u8]) -> Vec<Colorant> {
    let text = String::from_utf8_lossy(packet);
    let Some(start) = text.find("<xmpTPg:Colorants") else {
        return Vec::new();
    };
    let list = &text[start..];
    let list = &list[..list.find("</xmpTPg:Colorants").unwrap_or(list.len())];
    let mut out = Vec::new();
    for item in list.split("<rdf:li").skip(1) {
        let (Some(name), Some(mode)) = (
            property(item, "xmpG:swatchName"),
            property(item, "xmpG:mode"),
        ) else {
            continue;
        };
        let components: &[&str] = match mode.as_str() {
            "CMYK" => &["cyan", "magenta", "yellow", "black"],
            "RGB" => &["red", "green", "blue"],
            "LAB" => &["L", "A", "B"],
            _ => &[],
        };
        let values = components
            .iter()
            .filter_map(|c| property(item, &format!("xmpG:{c}"))?.trim().parse().ok())
            .collect();
        out.push(Colorant {
            name,
            mode,
            kind: property(item, "xmpG:type").unwrap_or_default(),
            tint: property(item, "xmpG:tint").is_some(),
            values,
        });
    }
    out
}

/// The name the packet gives the black swatch: the one colorant that is
/// not a tint, is a process colour, has the mode and values of `black`
/// (as IDML writes them: CMYK in percent, RGB 0–255) and whose name no
/// other colour of the document (`others`) has. `None` if there is no
/// such colorant or more than one.
pub fn black_name(
    colorants: &[Colorant],
    black: (&str, &[f64]),
    others: &[&str],
) -> Option<String> {
    let (mode, values) = black;
    let mut found = colorants.iter().filter(|c| {
        !c.tint
            && c.kind.eq_ignore_ascii_case("process")
            && c.mode == mode
            && c.values.len() == values.len()
            && c.values
                .iter()
                .zip(values)
                .all(|(a, b)| (a - b).abs() < 1e-3)
            && !others.contains(&c.name.as_str())
    });
    let first = found.next()?;
    found.next().is_none().then(|| first.name.clone())
}

/// A date `YYYY-MM-DDTHH:MM[:SS[.fff]]` followed by `Z` or `±HH:MM`.
fn parse(s: &str) -> Option<XmpDate> {
    let b = s.as_bytes();
    let num = |from: usize, len: usize| -> Option<i64> {
        let t = s.get(from..from + len)?;
        t.bytes()
            .all(|c| c.is_ascii_digit())
            .then(|| t.parse().ok())?
    };
    let (year, month, day) = (num(0, 4)?, num(5, 2)?, num(8, 2)?);
    if b.get(4) != Some(&b'-') || b.get(7) != Some(&b'-') || b.get(10) != Some(&b'T') {
        return None;
    }
    let (hour, minute) = (num(11, 2)?, num(14, 2)?);
    let mut i = 16;
    let mut second = 0;
    if b.get(i) == Some(&b':') {
        second = num(17, 2)?;
        i = 19;
        if b.get(i) == Some(&b'.') {
            i += 1;
            while b.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
        }
    }
    let offset = match b.get(i)? {
        b'Z' => 0,
        &c @ (b'+' | b'-') => {
            let v = num(i + 1, 2)? * 3600 + num(i + 4, 2)? * 60;
            if c == b'-' { -v } else { v }
        }
        _ => return None,
    };
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 {
        return None;
    }
    let local = days_from_civil(year, month, day) * 86_400 + hour * 3600 + minute * 60 + second;
    Some(XmpDate {
        utc: local - offset,
        offset,
    })
}

/// Days since 1970-01-01 of a date in the proleptic Gregorian calendar.
///
/// `chrono`'s `NaiveDate` only reaches about year ±262143, but the Gregorian
/// calendar repeats every 400 years (`DAYS_PER_400_YEARS`), so `chrono`
/// converts the year within its cycle and the whole cycles are added back as
/// days; the result matches the proleptic Gregorian count for every year, the
/// way the former Howard-Hinnant arithmetic did. All the intermediates are
/// `i128` so a near-`i64::MAX` year can't overflow; a count that then doesn't
/// fit `i64` (no representable answer) reads as 0.
///
/// A day past the month's length (a malformed XMP date that `parse` lets
/// through, e.g. "February 31") carries forward from the first of the month,
/// exactly as the former arithmetic did (it was linear in the day).
pub fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    use chrono::Datelike;
    let (cycles, y400) = (i128::from(year.div_euclid(400)), year.rem_euclid(400));
    // `y400` is 0..=399 and `month` 1..=12, so this is always in `chrono`'s range.
    let base = i32::try_from(y400).ok().and_then(|y| {
        u32::try_from(month)
            .ok()
            .and_then(|m| chrono::NaiveDate::from_ymd_opt(y, m, 1))
    });
    match base {
        Some(first) => {
            let days = i128::from(first.num_days_from_ce()) - i128::from(UNIX_EPOCH_DAYS_FROM_CE)
                + cycles * i128::from(DAYS_PER_400_YEARS)
                + i128::from(day)
                - 1;
            i64::try_from(days).unwrap_or(0)
        }
        None => 0,
    }
}

/// `NaiveDate::num_days_from_ce` of 1970-01-01, so the count can be rebased
/// onto the Unix epoch.
const UNIX_EPOCH_DAYS_FROM_CE: i64 = 719_163;

/// Days in one 400-year Gregorian cycle; the calendar repeats after it.
const DAYS_PER_400_YEARS: i64 = 146_097;

/// The date (year, month, day) of a day count since 1970-01-01, for any `i64`.
///
/// `chrono`'s `NaiveDate` only reaches about year ±262143, so `chrono` converts
/// the day within its 400-year Gregorian cycle (`DAYS_PER_400_YEARS`) and the
/// whole cycles are added back as years (an `i128` so `i64::MAX`/`MIN` can't
/// overflow); the result matches the proleptic Gregorian date for every count,
/// the way the former Howard-Hinnant arithmetic did.
pub fn civil_from_days(days: i64) -> (i64, i64, i64) {
    use chrono::Datelike;
    // CE day number of the date, in `i128` so no `days` overflows.
    let from_ce = i128::from(days) + i128::from(UNIX_EPOCH_DAYS_FROM_CE);
    let per_cycle = i128::from(DAYS_PER_400_YEARS);
    let (cycles, rest) = (from_ce.div_euclid(per_cycle), from_ce.rem_euclid(per_cycle));
    // `rest` is 0..146_097, a CE day inside years 0..=400, always in `chrono`'s range.
    let date = i32::try_from(rest)
        .ok()
        .and_then(chrono::NaiveDate::from_num_days_from_ce_opt);
    match date {
        Some(d) => {
            let year = i128::from(d.year()) + cycles * 400;
            let year = i64::try_from(year).unwrap_or(if year < 0 { i64::MIN } else { i64::MAX });
            (year, i64::from(d.month()), i64::from(d.day()))
        }
        // Unreachable: every `rest` in 0..146_097 is a valid CE day.
        None => (1970, 1, 1),
    }
}

/// The day of the year (0-based) of an instant, in UTC.
pub fn day_of_year(utc: i64) -> i64 {
    let days = utc.div_euclid(86_400);
    let (year, _, _) = civil_from_days(days);
    days - days_from_civil(year, 1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_dates_with_offsets() {
        let packet = br#"<x xmp:CreateDate="2020-03-01T10:00:00+01:00" xmp:ModifyDate="2020-07-01T10:00:00.25-04:00">
<stEvt:when>2021-01-02T03:04:05Z</stEvt:when><xmp:MetadataDate>2020-01-01T00:00</xmp:MetadataDate></x>"#;
        let d = dates(packet);
        assert_eq!(d.len(), 3);
        assert_eq!(d[0].offset, 3600);
        assert_eq!(d[0].utc, days_from_civil(2020, 3, 1) * 86_400 + 9 * 3600);
        assert_eq!(d[1].offset, -4 * 3600);
        assert_eq!(d[2].offset, 0);
    }

    #[test]
    fn finds_the_black_name() {
        let packet = r#"<xmpTPg:Colorants><rdf:Seq>
<rdf:li rdf:parseType="Resource"><xmpG:swatchName>Schwarz</xmpG:swatchName><xmpG:mode>CMYK</xmpG:mode><xmpG:type>PROCESS</xmpG:type><xmpG:cyan>0</xmpG:cyan><xmpG:magenta>0</xmpG:magenta><xmpG:yellow>0</xmpG:yellow><xmpG:black>100</xmpG:black></rdf:li>
<rdf:li xmpG:swatchName="Schwarz" xmpG:mode="CMYK" xmpG:type="PROCESS" xmpG:tint="40" xmpG:cyan="0" xmpG:magenta="0" xmpG:yellow="0" xmpG:black="100"/>
<rdf:li xmpG:swatchName="Ink &amp; More" xmpG:mode="CMYK" xmpG:type="PROCESS" xmpG:cyan="0" xmpG:magenta="0" xmpG:yellow="0" xmpG:black="100"/>
</rdf:Seq></xmpTPg:Colorants>"#;
        let c = colorants(packet.as_bytes());
        assert_eq!(c.len(), 3);
        assert!(c[1].tint);
        assert_eq!(c[2].name, "Ink & More");
        let black = ("CMYK", &[0.0, 0.0, 0.0, 100.0][..]);
        // Two candidates: ambiguous.
        assert_eq!(black_name(&c, black, &[]), None);
        assert_eq!(
            black_name(&c, black, &["Ink & More"]).as_deref(),
            Some("Schwarz")
        );
    }

    #[test]
    fn converts_between_days_and_dates() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(civil_from_days(days_from_civil(2024, 2, 29)), (2024, 2, 29));
        assert_eq!(civil_from_days(days_from_civil(1601, 1, 1)), (1601, 1, 1));
        assert_eq!(day_of_year(days_from_civil(2023, 12, 31) * 86_400), 364);
        // Past chrono's own ±262143 date range the 400-year cycles carry the year, so accepted
        // counts keep the dates the pre-chrono (Hinnant) arithmetic gave. Values from the old
        // formula, in both directions and on both sides of the epoch.
        for (days, date) in [
            (95_026_602, (262_144, 1, 1)),
            (108_853_222, (300_000, 1, 1)),
            (100_000_000, (275_760, 9, 13)),
            (-96_465_659, (-262_145, 12, 31)),
            (-110_292_278, (-300_000, 1, 1)),
            (-100_000_000, (-271_821, 4, 20)),
        ] {
            assert_eq!(civil_from_days(days), date, "{days}");
            assert_eq!(days_from_civil(date.0, date.1, date.2), days, "{date:?}");
        }
        // The day still carries linearly past chrono's range (a malformed "February 31").
        assert_eq!(
            civil_from_days(days_from_civil(300_000, 2, 31)),
            (300_000, 3, 2)
        );
        // 300000 is a leap year (divisible by 400), so 300000-12-31 is 0-based day 365.
        assert_eq!(day_of_year(days_from_civil(300_000, 12, 31) * 86_400), 365);
        // The i64 extremes stay exact (i128 intermediates) rather than panicking or clamping.
        assert_eq!(civil_from_days(i64::MAX), (25_252_734_927_768_524, 7, 27));
        assert_eq!(civil_from_days(i64::MIN), (-25_252_734_927_764_585, 6, 7));
    }
}
