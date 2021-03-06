use crate::tokenization::Text;
use crate::search::Hit;


pub fn highlight(hit: &Hit, dividers: (&[char], &[char])) -> String {
    let (div_left, div_right) = dividers;
    let Hit {
        title: Text { words, source, .. },
        rmatches,
        ..
    } = hit;

    let mut highlighted = {
        let chars_src = source.len();
        let chars_hl  = (div_left.len() + div_right.len() + 1) * words.len();
        String::with_capacity((chars_src + chars_hl) * 4)
    };

    let mut char_offset = 0;
    for (word_offset, word) in words.iter().enumerate() {
        match rmatches.iter().find(|m| m.offset == word_offset) {
            Some(rmatch) => {
                let match_start = word.slice.0 + rmatch.subslice.0;
                let match_end   = word.slice.0 + rmatch.subslice.1;
                highlighted.extend(&source[char_offset .. match_start]);
                highlighted.extend(div_left);
                highlighted.extend(&source[match_start .. match_end]);
                highlighted.extend(div_right);
                highlighted.extend(&source[match_end .. word.slice.1]);
            },
            None => {
                highlighted.extend(&source[char_offset .. word.slice.1]);
            },
        }
        char_offset = word.slice.1;
    }
    highlighted.extend(&source[char_offset .. ]);
    highlighted.retain(|ch| ch != '\0');

    highlighted
}


#[cfg(test)]
mod tests {
    use crate::matching::WordMatch;
    use crate::store::Record;
    use crate::search::Hit;
    use crate::lang::{Lang, lang_german, lang_portuguese};
    use super::highlight;

    const L: &[char] = &['['];
    const R: &[char] = &[']'];

    fn mock_match(offset: usize, size: usize) -> (WordMatch, WordMatch) {
        let rmatch = WordMatch {
            offset:   offset,
            slice:    (0, size),
            subslice: (0, size),
            func:     false,
            typos:    0.0,
            fin:      false,
        };
        let qmatch = WordMatch {
            offset:   0,
            slice:    (0, size),
            subslice: (0, size),
            func:     false,
            typos:    0.0,
            fin:      false,
        };
        (rmatch, qmatch)
    }

    #[test]
    fn highlight_basic() {
        let lang   = Lang::new();
        let record = Record::new(10, "metal detector", 0, &lang);

        let mut hit = Hit::from_record(&record);
        let (rmatch, qmatch) = mock_match(1, 6);
        hit.rmatches.push(rmatch);
        hit.qmatches.push(qmatch);

        let expected = "metal [detect]or";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_stripped() {
        let lang   = Lang::new();
        let record = Record::new(10, "'metal' mailbox!", 0, &lang);

        let mut hit = Hit::from_record(&record);
        let (rmatch, qmatch) = mock_match(0, 5);
        hit.rmatches.push(rmatch);
        hit.qmatches.push(qmatch);

        let expected = "'[metal]' mailbox!";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_multichar_dividers() {
        let lang   = Lang::new();
        let record = Record::new(10, "metal detector", 0, &lang);

        let mut hit = Hit::from_record(&record);
        let (rmatch, qmatch) = mock_match(1, 6);
        hit.rmatches.push(rmatch);
        hit.qmatches.push(qmatch);

        let l: &[char] = &['{', '{'];
        let r: &[char] = &['}', '}'];

        let expected = "metal {{detect}}or";
        let received = highlight(&hit, (l, r));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_utf_padded() {
        let lang   = lang_german();
        let record = Record::new(10, "Passstraße", 0, &lang);

        let mut hit = Hit::from_record(&record);
        let (rmatch, qmatch) = mock_match(0, 9);
        hit.rmatches.push(rmatch);
        hit.qmatches.push(qmatch);

        let expected = "[Passstraß]e";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_utf_nfd() {
        let lang   = lang_portuguese();
        let record = Record::new(10, "Passstraße", 0, &lang);

        let mut hit = Hit::from_record(&record);
        let (rmatch, qmatch) = mock_match(0, 9);
        hit.rmatches.push(rmatch);
        hit.qmatches.push(qmatch);

        let expected = "[Passstraß]e";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }
}
