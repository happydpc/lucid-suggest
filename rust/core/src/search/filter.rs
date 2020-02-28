use crate::lexis::Text;
use crate::search::Hit;


pub struct Filter<'a, Src: Iterator<Item=Hit<'a>>> {
    source: Src,
    query:  &'a Text<&'a [char]>,
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Filter<'a, Src> {
    pub fn new(source: Src, query:  &'a Text<&'a [char]>,) -> Self {
        Self { source, query }
    }
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Iterator for Filter<'a, Src> {
    type Item = Hit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { source, query } = self;
        loop {
            match source.next() {
                Some(hit) if matches(query, &hit) => {
                    return Some(hit);
                },
                Some(_) => {
                    continue;
                },
                None => {
                    return None;
                },
            }
        }
    }
}


fn matches(query: &Text<&[char]>, hit: &Hit) -> bool {
    let matches = &hit.scores.matches;
    if query.is_empty() { return true; }
    if matches.len() == 0 { return false; }
    if matches.len() == 1 && query.words.len() > 1 {
        let word_match = &matches[0];
        let unfinished = !word_match.fin;
        let first_half = (word_match.query.len * 2) < word_match.record.len;
        if unfinished && first_half { return false; }
    }
    true
}