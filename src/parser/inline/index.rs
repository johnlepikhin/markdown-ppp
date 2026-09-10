//! Per-slice index of delimiter positions for the inline parser.
//!
//! Several inline parsers need to know where a construct that starts at the current
//! position ends: the closing `*`/`_` run of an emphasis, the `]` matching a `[`, the
//! end of a link destination. Scanning forward from every candidate position makes
//! the parse quadratic in the input size (`*a *a *a ...`, `[[[[...`, `[a]([a]([a](...`).
//!
//! An [`InlineIndex`] is built once per slice handed to `inline_many0`/`inline_many1`
//! (a paragraph, the content of an emphasis, a link label, ...) in a single linear
//! pass, after which every lookup is `O(log n)` or amortized `O(1)`. The index is
//! keyed by byte offset from the start of the slice; parsers pass the sub-slice they
//! are looking at and the index derives the offset from the pointer.

use crate::parser::inline::emphasis::can_close;
use std::cell::RefCell;

/// Sentinel in [`InlineIndex::destination_ends`] for an offset whose end is unknown.
const UNKNOWN: usize = usize::MAX;

/// Bytes the index builder has to look at: delimiters it records, the backslash,
/// and every byte that cannot be part of a link destination (which resets the
/// paren stack). Everything else is skipped in bulk.
const NEEDS_ATTENTION: [bool; 256] = {
    let mut t = [false; 256];
    let mut b = 0usize;
    while b < 256 {
        t[b] = matches!(
            b as u8,
            b'\\' | b'*' | b'_' | b'[' | b']' | b'(' | b')' | b'"' | b'\''
        ) || !is_destination_byte(b as u8);
        b += 1;
    }
    t
};

/// [`is_destination_char`] for a single byte; non-ASCII bytes are destination bytes.
const fn is_destination_byte(b: u8) -> bool {
    !(b.is_ascii_control() || b == b' ' || b == b'<')
}

/// Emphasis delimiter runs, longest first (the order the emphasis parser tries them).
pub(crate) const EMPHASIS_TAGS: [&str; 6] = ["***", "___", "**", "__", "*", "_"];

/// Maximum nesting depth of square brackets inside a link label; deeper `[` are
/// literal characters. Mirrors the limit of the recursive scanner in `link_util`.
const MAX_BRACKET_DEPTH: usize = 32;

pub(crate) struct InlineIndex {
    base: usize,
    len: usize,
    /// For every emphasis tag, sorted offsets at which a valid closing tag starts.
    closers: [Vec<usize>; 6],
    /// `(offset of '[', offset of the matching ']')`, sorted by the first field.
    brackets: Vec<(usize, usize)>,
    /// Sorted offsets of every `]`.
    close_brackets: Vec<usize>,
    /// `(offset of '(', offset of the matching ')')`, sorted by the first field.
    parens: Vec<(usize, usize)>,
    /// Sorted offsets of every unescaped `"`, `'` and `)` (link title delimiters).
    title_ends: [Vec<usize>; 3],
    /// Memoized end offsets of link destinations, one slot per byte of the slice
    /// (`UNKNOWN` until computed); allocated on first use. See [`Self::destination_len`].
    destination_ends: RefCell<Vec<usize>>,
}

impl InlineIndex {
    pub(crate) fn build(input: &str) -> Self {
        let bytes = input.as_bytes();

        let mut closers: [Vec<usize>; 6] = Default::default();
        let mut brackets = Vec::new();
        let mut close_brackets = Vec::new();
        let mut parens = Vec::new();
        let mut title_ends: [Vec<usize>; 3] = Default::default();
        let mut bracket_stack: Vec<usize> = Vec::new();
        let mut paren_stack: Vec<usize> = Vec::new();

        // The bracket and paren scanners treat `\x` as an opaque escape pair.
        let mut escaped = false;
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            // Fast path: a byte that is neither a delimiter nor a destination
            // breaker needs no bookkeeping. Skipping the run here avoids the
            // (poorly predicted) dispatch below for the bulk of the text.
            if !escaped && !NEEDS_ATTENTION[b as usize] {
                i += 1;
                while i < bytes.len() && !NEEDS_ATTENTION[bytes[i] as usize] {
                    i += 1;
                }
                continue;
            }
            if escaped {
                escaped = false;
                if b == b'*' || b == b'_' {
                    // Emphasis: only `\*` is an escape, `\_` may still close.
                    push_closers(&mut closers, input, i, true);
                }
                i += 1;
                continue;
            }
            match b {
                b'\\' => escaped = true,
                b'*' | b'_' => push_closers(&mut closers, input, i, false),
                b'[' => {
                    if bracket_stack.len() <= MAX_BRACKET_DEPTH {
                        bracket_stack.push(i);
                    }
                }
                b']' => {
                    close_brackets.push(i);
                    if let Some(open) = bracket_stack.pop() {
                        brackets.push((open, i));
                    }
                }
                b'(' => paren_stack.push(i),
                b')' => {
                    title_ends[2].push(i);
                    if let Some(open) = paren_stack.pop() {
                        parens.push((open, i));
                    }
                }
                b'"' => title_ends[0].push(i),
                b'\'' => title_ends[1].push(i),
                _ => {
                    // A character that cannot be part of a link destination breaks
                    // every open paren group.
                    if !is_destination_char(b as char) {
                        paren_stack.clear();
                    }
                }
            }
            i += 1;
        }

        // Pairs are pushed in closing order; lookups binary-search by the opener.
        brackets.sort_unstable_by_key(|&(open, _)| open);
        parens.sort_unstable_by_key(|&(open, _)| open);

        Self {
            base: input.as_ptr() as usize,
            len: input.len(),
            closers,
            brackets,
            close_brackets,
            parens,
            title_ends,
            destination_ends: RefCell::new(Vec::new()),
        }
    }

    /// Offset (relative to `at`) of the first unescaped `delim` (one of `"`, `'`, `)`)
    /// strictly after the first byte of `at`; `None` when there is none within `at`.
    /// Returns `None` also when `at` is not covered; callers must then scan.
    pub(crate) fn title_end(&self, at: &str, delim: u8) -> Option<Option<usize>> {
        let from = self.offset(at)?;
        let list = match delim {
            b'"' => &self.title_ends[0],
            b'\'' => &self.title_ends[1],
            b')' => &self.title_ends[2],
            _ => return None,
        };
        let idx = list.partition_point(|&c| c <= from);
        let Some(&close) = list.get(idx) else {
            return Some(None);
        };
        Some((close < from + at.len()).then_some(close - from))
    }

    /// Byte offset of `s` within the indexed slice, or `None` if `s` is not a
    /// sub-slice of it.
    fn offset(&self, s: &str) -> Option<usize> {
        let ptr = s.as_ptr() as usize;
        if ptr < self.base || ptr + s.len() > self.base + self.len {
            return None;
        }
        Some(ptr - self.base)
    }

    /// Length of the content of an emphasis whose content starts at `content`, i.e.
    /// the distance to the first valid closing `tag`. `None` when there is no closer
    /// within `content` or the content would be empty. `None` is also returned when
    /// `content` is not covered by the index; callers must then fall back to a scan.
    pub(crate) fn emphasis_content_len(&self, content: &str, tag: &str) -> Option<Option<usize>> {
        let from = self.offset(content)?;
        let k = EMPHASIS_TAGS.iter().position(|t| *t == tag)?;
        let list = &self.closers[k];
        let idx = list.partition_point(|&c| c < from);
        let Some(&closer) = list.get(idx) else {
            return Some(None);
        };
        if closer >= from + content.len() || closer == from {
            return Some(None);
        }
        Some(Some(closer - from))
    }

    /// Whether `at` is covered by this index.
    pub(crate) fn covers(&self, at: &str) -> bool {
        self.offset(at).is_some()
    }

    /// Offset (relative to `at`, which starts with `[`) of the `]` matching it.
    pub(crate) fn bracket_match(&self, at: &str) -> Option<usize> {
        let from = self.offset(at)?;
        let close = lookup(&self.brackets, from)?;
        (close < from + at.len()).then_some(close - from)
    }

    /// Offset (relative to `at`) of the first `]` at or after `at`.
    pub(crate) fn next_close_bracket(&self, at: &str) -> Option<usize> {
        let from = self.offset(at)?;
        let idx = self.close_brackets.partition_point(|&c| c < from);
        let close = *self.close_brackets.get(idx)?;
        (close < from + at.len()).then_some(close - from)
    }

    /// Length of the longest link destination (the form without `<...>`) starting at
    /// `at`: a run of destination characters, escape pairs and balanced paren groups
    /// whose content is itself such a run. Zero when no destination starts there.
    ///
    /// Ends are memoized per offset, and a walk stops as soon as it reaches an offset
    /// whose end is known, so the total work over a slice is linear.
    pub(crate) fn destination_len(&self, at: &str) -> usize {
        let Some(from) = self.offset(at) else {
            return destination_len_slow(at);
        };
        let limit = from + at.len();
        let mut memo = self.destination_ends.borrow_mut();
        if memo.is_empty() {
            memo.resize(self.len, UNKNOWN);
        }
        let mut pos = from;
        let end = loop {
            if pos >= limit {
                break limit;
            }
            let known = memo[pos];
            if known != UNKNOWN {
                break known.min(limit);
            }
            let rest = &at[pos - from..];
            let c = rest.chars().next().unwrap();
            match c {
                '\\' => pos += escape_len(rest),
                '(' => match lookup(&self.parens, pos) {
                    Some(close) if close < limit => pos = close + 1,
                    _ => break pos,
                },
                ')' => break pos,
                c if is_destination_char(c) => pos += c.len_utf8(),
                _ => break pos,
            }
        };
        for slot in &mut memo[from..pos.min(self.len)] {
            if *slot == UNKNOWN {
                *slot = end;
            }
        }
        end - from
    }
}

/// Value paired with `key` in a list sorted by key.
fn lookup(pairs: &[(usize, usize)], key: usize) -> Option<usize> {
    pairs
        .binary_search_by_key(&key, |&(open, _)| open)
        .ok()
        .map(|i| pairs[i].1)
}

fn push_closers(closers: &mut [Vec<usize>; 6], input: &str, i: usize, escaped: bool) {
    let rest = &input[i..];
    let marker = rest.chars().next().unwrap();
    if escaped && marker == '*' {
        return;
    }
    for (k, tag) in EMPHASIS_TAGS.iter().enumerate() {
        if tag.starts_with(marker) && rest.starts_with(tag) {
            let next = rest[tag.len()..].chars().next();
            if can_close(marker, next) {
                closers[k].push(i);
            }
        }
    }
}

/// Byte length of the escape pair starting at `rest` (which starts with `\`).
fn escape_len(rest: &str) -> usize {
    1 + rest[1..].chars().next().map_or(0, char::len_utf8)
}

/// Characters allowed in a link destination outside `<...>`.
pub(crate) fn is_destination_char(c: char) -> bool {
    !c.is_ascii_control() && c != ' ' && c != '<'
}

/// [`InlineIndex::destination_len`] for input that is not covered by an index.
/// Linear in the input, without recursion.
pub(crate) fn destination_len_slow(at: &str) -> usize {
    let bytes = at.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() {
        match bytes[pos] {
            b'\\' => pos += escape_len(&at[pos..]),
            b'(' => match paren_group_len(&at[pos..]) {
                Some(len) => pos += len,
                None => break,
            },
            b')' => break,
            // Bytes of a multi-byte character are never ASCII, so they are all
            // destination bytes and can be stepped over one at a time.
            b if is_destination_byte(b) => pos += 1,
            _ => break,
        }
    }
    pos
}

/// Length of a balanced paren group starting at `at` (which starts with `(`), whose
/// content consists of destination characters, escape pairs and nested groups.
fn paren_group_len(at: &str) -> Option<usize> {
    let bytes = at.as_bytes();
    let mut depth = 0usize;
    let mut pos = 0;
    while pos < bytes.len() {
        match bytes[pos] {
            b'\\' => pos += escape_len(&at[pos..]),
            b'(' => {
                depth += 1;
                pos += 1;
            }
            b')' => {
                depth -= 1;
                pos += 1;
                if depth == 0 {
                    return Some(pos);
                }
            }
            b if is_destination_byte(b) => pos += 1,
            _ => return None,
        }
    }
    None
}
