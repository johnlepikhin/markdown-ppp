use nom::IResult;

/// A code span: a run of backticks, content, and a closing run of the same length
/// that is not followed by another backtick. The content must not contain a blank
/// line and must not be empty.
pub(crate) fn code_span(input: &str) -> IResult<&str, String> {
    let error = || nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag));
    let bytes = input.as_bytes();
    let tick_count = bytes.iter().take_while(|&&b| b == b'`').count();
    if tick_count == 0 {
        return Err(error());
    }
    let body = &input[tick_count..];
    let bb = body.as_bytes();
    let mut pos = 0;
    let end = loop {
        // Skip to the next byte that matters: a backtick or a line ending.
        pos += bb[pos..]
            .iter()
            .position(|&b| b == b'`' || b == b'\n' || b == b'\r')
            .unwrap_or(bb.len() - pos);
        if pos >= bb.len() {
            return Err(error());
        }
        if bb[pos] == b'`' {
            let run = bb[pos..].iter().take_while(|&&b| b == b'`').count();
            if run >= tick_count {
                // A longer run closes at its last `tick_count` backticks; the
                // ones before belong to the content.
                let close = pos + run - tick_count;
                if close == 0 {
                    return Err(error());
                }
                break close;
            }
            pos += run;
            continue;
        }
        if starts_with_empty_line(bb, pos) {
            return Err(error());
        }
        pos += 1;
    };
    let rest = &body[end + tick_count..];

    // Line endings become spaces; one leading and one trailing space are stripped
    // when the content is not all spaces.
    let mut content = String::with_capacity(end);
    let mut i = 0;
    let raw = &bb[..end];
    while i < end {
        match raw[i] {
            b'\r' if raw.get(i + 1) == Some(&b'\n') => {
                content.push(' ');
                i += 2;
            }
            b'\n' => {
                content.push(' ');
                i += 1;
            }
            _ => {
                let c = body[i..].chars().next().unwrap();
                content.push(c);
                i += c.len_utf8();
            }
        }
    }
    if content.starts_with(' ') && content.ends_with(' ') && content.trim() != "" {
        content = content[1..content.len() - 1].to_string();
    }

    Ok((rest, content))
}

/// A line ending, optional blanks, and another line ending start at `pos`.
fn starts_with_empty_line(bb: &[u8], pos: usize) -> bool {
    let mut i = pos;
    let Some(next) = line_ending_len(bb, i) else {
        return false;
    };
    i += next;
    while i < bb.len() && (bb[i] == b' ' || bb[i] == b'\t') {
        i += 1;
    }
    line_ending_len(bb, i).is_some()
}

fn line_ending_len(bb: &[u8], pos: usize) -> Option<usize> {
    match bb.get(pos) {
        Some(b'\n') => Some(1),
        Some(b'\r') if bb.get(pos + 1) == Some(&b'\n') => Some(2),
        _ => None,
    }
}
