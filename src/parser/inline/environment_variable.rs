use crate::ast::Inline;
use nom::IResult;

/// Longest identifier accepted as an environment variable.
pub(crate) const MAX_ENV_VAR_LEN: usize = 50;

const IS_WORD_BYTE: [bool; 256] = {
    let mut t = [false; 256];
    let mut b = 0usize;
    while b < 256 {
        t[b] = (b as u8).is_ascii_alphanumeric() || b as u8 == b'_';
        b += 1;
    }
    t
};

/// The `[A-Za-z0-9_]+` word starting at `rest`, looking at most `limit` bytes: its
/// length (capped at `limit`) and the position of its first `_` within that window.
fn scan_word_within(rest: &str, limit: usize) -> (usize, Option<usize>) {
    let bytes = rest.as_bytes();
    let limit = bytes.len().min(limit);
    let mut underscore = None;
    let mut i = 0;
    while i < limit && IS_WORD_BYTE[bytes[i] as usize] {
        if bytes[i] == b'_' && underscore.is_none() {
            underscore = Some(i);
        }
        i += 1;
    }
    (i, underscore)
}

/// The `[A-Za-z0-9_]+` word starting at `rest`: its length and the position of its
/// first `_`, if any. Only a bounded window is looked at, so that a long word is
/// not rescanned from every one of its characters: anything longer than
/// `MAX_ENV_VAR_LEN` is reported as `MAX_ENV_VAR_LEN + 1` (with no underscore).
pub(crate) fn scan_word(rest: &str) -> (usize, Option<usize>) {
    let (len, underscore) = scan_word_within(rest, MAX_ENV_VAR_LEN + 1);
    if len > MAX_ENV_VAR_LEN {
        return (len, None);
    }
    (len, underscore)
}

/// Like [`scan_word`] but unbounded: the whole word, however long.
pub(crate) fn scan_word_full(rest: &str) -> (usize, Option<usize>) {
    scan_word_within(rest, usize::MAX)
}

/// Whether the word is short enough for [`is_likely_env_var`], given [`scan_word`]'s
/// length.
pub(crate) fn fits(len: usize) -> bool {
    len <= MAX_ENV_VAR_LEN
}

/// An identifier that looks like an environment variable (`FOO_BAR`), as plain text,
/// so that its underscores are not taken as emphasis markers.
pub(crate) fn environment_variable(input: &str) -> IResult<&str, Inline> {
    let fail = || nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify));
    if !input
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphabetic)
    {
        return Err(fail());
    }
    let (len, underscore) = scan_word(input);
    if !fits(len) || underscore.is_none() || !is_likely_env_var(&input[..len]) {
        return Err(fail());
    }
    Ok((&input[len..], Inline::Text(input[..len].to_string())))
}

/// Whether an `[A-Za-z][A-Za-z0-9_]*` word looks like an environment variable
/// (`FOO_BAR`): it has an underscore, but not at either end and never doubled.
pub(crate) fn is_likely_env_var(s: &str) -> bool {
    // Must contain at least one underscore
    if !s.contains('_') {
        return false;
    }

    // Must not start or end with underscore
    if s.starts_with('_') || s.ends_with('_') {
        return false;
    }

    // Must not have consecutive underscores
    if s.contains("__") {
        return false;
    }

    // Should be reasonable length (heuristic)
    if s.len() < 3 || s.len() > 50 {
        return false;
    }

    true
}
