//! Searches for names stored as in-object strings at positions that are
//! not decoded: layer, style and table style names.
//!
//! Evidence: `docs/format/objects.md`.

use super::*;

/// Find a flag byte (1 = built-in name) followed by an in-object string
/// that `accept` takes, at or after `from`. Returns the offset of the pair,
/// whether the flag is 1, and the string. In big-endian data the flag and
/// the string's tag (2) are swapped (`docs/format/big-endian.md`).
pub(super) fn find_flagged_string(
    enc: Encoding,
    data: &[u8],
    from: usize,
    accept: impl Fn(&str) -> bool,
) -> Option<(usize, bool, String)> {
    let big = enc.big_endian();
    (from..data.len().saturating_sub(6)).find_map(|i| {
        let header = match big {
            // InDesign 2.0 strings start with 1 (`Encoding::string_tag`).
            false => data[i] <= 2 && data[i + 1] == enc.string_tag(),
            true => data[i] == 2 && data[i + 1] <= 2,
        };
        if !header {
            return None;
        }
        let mut c = enc.cursor(&data[i..]);
        let n = c.name().ok()?;
        // With tag 1 the pattern also matches zero bytes before the real
        // string and reads an empty one (objects.md, styles).
        if enc.indesign_2() && n.name.is_empty() {
            return None;
        }
        accept(&n.name).then_some((i, n.builtin, n.name))
    })
}

/// Find the first in-object string at or after `from`. The byte after
/// the tag can have any value (`docs/format/objects.md`, in-object
/// strings).
pub(super) fn find_string(enc: Encoding, data: &[u8], from: usize) -> Result<String, Error> {
    if enc.big_endian() {
        // The tag (2), the byte before it in little-endian data, then the
        // rest of the header (`Cursor::string`).
        for i in from..data.len().saturating_sub(5) {
            if data[i] == 2
                && data[i + 1] <= 2
                && enc.u16_from([data[i + 3], data[i + 4]]) > 0
                && let Ok(s) = enc.cursor(&data[i..]).string()
            {
                return Ok(s);
            }
        }
        return Ok(String::new());
    }
    for i in from..data.len().saturating_sub(4) {
        if data[i] == 2 {
            let n = enc.u16_from([data[i + 2], data[i + 3]]) as usize;
            if n > 0
                && i + 6 <= data.len()
                && (enc.u16_from([data[i + 4], data[i + 5]]) & 0xC000) != 0
                && let Ok(s) = enc.cursor(&data[i..]).string()
            {
                return Ok(s);
            }
        }
    }
    Ok(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_flagged_names_of_indesign_2() {
        use crate::database::synthetic;
        use crate::header::{ByteOrder, Version};
        let v2 = Encoding::new(ByteOrder::Little, Version { major: 2, minor: 0 });
        // Chunk 0x230 of a root paragraph style: next and based-on 0, u32
        // 0, u32 0, u16 1 (paragraph), u16 0, then the built-in name. Bytes
        // 15 to 19 (0, 1, 0, 0, 0) look like an empty tag-1 string.
        let mut d = vec![0u8; 16];
        d.extend_from_slice(&[1, 0, 0, 0]);
        d.extend(synthetic::flagged_string(v2, 1, "[No paragraph style]"));
        assert_eq!(
            find_flagged_string(v2, &d, 12, |_| true),
            Some((20, true, "[No paragraph style]".to_string()))
        );
        // Files from 3.0 on: tag 2 only, as before.
        assert_eq!(
            find_flagged_string(Encoding::default(), &d, 12, |_| true),
            None
        );
    }

    #[test]
    fn finds_strings_with_any_second_byte() {
        let enc = Encoding::default();
        for second in [0, 1, 0x19] {
            let d = [9, 9, 2, second, 2, 0, 2, 0x40, b'a', b'b'];
            assert_eq!(find_string(enc, &d, 0).unwrap(), "ab");
        }
    }
}
