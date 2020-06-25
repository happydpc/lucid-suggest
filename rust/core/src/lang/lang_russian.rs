#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;
use super::utils::compile_utf_map;


const ARTICLES: [&'static str; 0] = [

];

const PREPOSITIONS: [&'static str; 61] = [
    "c",
    "без",
    "благодаря",
    "в",
    "ввиду",
    "вдоль",
    "вместо",
    "вне",
    "внутри",
    "внутрь",
    "во",
    "возле",
    "вокруг",
    "вопреки",
    "впереди",
    "вследствие",
    "для",
    "до",
    "за",
    "из-за",
    "из-под",
    "из",
    "к",
    "ко",
    "кроме",
    "между",
    "мимо",
    "на",
    "над",
    "надо",
    "напротив",
    "насчет",
    "о",
    "об",
    "около",
    "от",
    "ото",
    "перед",
    "передо",
    "по",
    "под",
    "подле",
    "подо",
    "позади",
    "помимо",
    "после",
    "посреди",
    "посредством",
    "при",
    "про",
    "против",
    "путём",
    "ради",
    "с",
    "сверх",
    "свыше",
    "сквозь",
    "со",
    "среди",
    "у",
    "через",
];

const CONJUNCTIONS: [&'static str; 51] = [
    "а",
    "а",
    "будто",
    "впрочем",
    "где",
    "да",
    "едва",
    "ежели",
    "если",
    "же",
    "зато",
    "и",
    "ибо",
    "или",
    "именно",
    "как",
    "как",
    "когда",
    "которая",
    "которого",
    "которое",
    "котором",
    "которую",
    "которые",
    "который",
    "которых",
    "ли",
    "либо",
    "лишь",
    "настолько",
    "но",
    "однако",
    "пока",
    "покамест",
    "покуда",
    "пускай",
    "пусть",
    "раз",
    "словно",
    "также",
    "то",
    "тоже",
    "точно",
    "хоть",
    "хотя",
    "чем",
    "что",
    "что",
    "чтобы",
    "чтобы",
    "чуть",
    // "а именно",
    // "а то",
    // "благодаря тому что",
    // "будто бы",
    // "в то время как",
    // "ввиду того что",
    // "всё же",
    // "да нет",
    // "для того чтобы",
    // "до того как",
    // "до того",
    // "ещё не ... как",
    // "и ... и",
    // "или ... или",
    // "как ... ни",
    // "как будто бы",
    // "как будто",
    // "как бы",
    // "как ни",
    // "как только",
    // "либо ... либо",
    // "между тем как",
    // "настолько ... насколько",
    // "не ... как",
    // "не настолько ... насколько",
    // "не так ... как",
    // "не то ... не то",
    // "не то что",
    // "не то чтобы",
    // "не только ... но и",
    // "несмотря на то что",
    // "ни ... ни",
    // "оттого что",
    // "перед тем как",
    // "пока .. не",
    // "пока не",
    // "после того как",
    // "потому что",
    // "правда ... но",
    // "прежде чем",
    // "разве только",
    // "разве что",
    // "с тех пор как",
    // "так как",
    // "так что",
    // "то ... то",
    // "то есть",
    // "чем ... тем",
    // "чтобы не",
];

const PARTICLES: [&'static str; 32] = [
    "будто",
    "бы",
    "ведь",
    "ведь",
    "вон",
    "вот",
    "да",
    "даже",
    "есть",
    "же",
    "здесь",
    "именно",
    "лишь",
    "не",
    "нет",
    "неужели",
    "ни",
    "ну",
    "ну",
    "пожалуй",
    "почти",
    "просто",
    "разве",
    "словно",
    "там",
    "только",
    "угодно",
    "уж",
    "хоть",
    "хотя",
    "чуть",
    "это",
    // "вряд ли",
    // "да ну",
    // "даже и",
    // "едва ли",
    // "ещё бы",
    // "как будто",
    // "как раз",
    // "лишь только",
    // "ни ... ни",
    // "ну да",
    // "ну и",
    // "только лишь",
    // "хоть бы",
    // "хотя бы",
    // "чуть ли не",
    // "чуть ли",
    // "чуть не",
];

const UTF_COMPOSE_MAP: [(&'static str, &'static str); 2] = [
    ("Ё", "Ё"),
    ("ё", "ё"),
];

const UTF_REDUCE_MAP: [(&'static str, &'static str); 2] = [
    ("Ё", "Е"),
    ("ё", "е"),
];


pub fn lang_russian() -> Lang {
    let stemmer = Stemmer::create(Algorithm::Russian);

    let compose_map = compile_utf_map(&UTF_COMPOSE_MAP[..]);
    let reduce_map  = compile_utf_map(&UTF_REDUCE_MAP[..]);

    let mut pos_map = HashMap::new();
    for w in &ARTICLES[..]     { pos_map.insert(w.chars().collect(), PartOfSpeech::Article); }
    for w in &PREPOSITIONS[..] { pos_map.insert(w.chars().collect(), PartOfSpeech::Preposition); }
    for w in &CONJUNCTIONS[..] { pos_map.insert(w.chars().collect(), PartOfSpeech::Conjunction); }
    for w in &PARTICLES[..]    { pos_map.insert(w.chars().collect(), PartOfSpeech::Particle); }

    Lang::new(pos_map, compose_map, reduce_map, stemmer)
}


#[cfg(test)]
mod tests {
    use crate::tokenization::PartOfSpeech;
    use super::{lang_russian, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_russian();
        let w = "важный".chars().collect::<Vec<_>>();
        assert_eq!(lang.stem(&w), 4);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_russian();
        let w1 = "важный".chars().collect::<Vec<_>>();
        let w2 = "ведь"  .chars().collect::<Vec<_>>();
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Particle));
    }

    #[test]
    fn utf_compose() {
        let lang = lang_russian();

        let source1 = "важный";
        let norm1   = lang.utf_compose(&source1.chars().collect::<Vec<_>>());
        assert_eq!(norm1, None);

        let source2 = "Ёлка";
        let norm2   = lang
            .utf_compose(&source2.chars().collect::<Vec<_>>())
            .unwrap()
            .iter()
            .collect::<String>();
        assert_eq!(norm2, "Ёлка");
        assert_eq!(norm2.chars().count(), source2.chars().count() - 1);
    }

    #[test]
    fn utf_reduce() {
        let lang = lang_russian();

        let source1 = "важный";
        let norm1   = lang.utf_reduce(&source1.chars().collect::<Vec<_>>());
        assert_eq!(norm1, None);

        let source2 = "Ёлка";
        let (padded2, norm2) = lang
            .utf_reduce(&source2.chars().collect::<Vec<_>>())
            .unwrap();
        let padded2 = padded2.iter().collect::<String>();
        let norm2   = norm2  .iter().collect::<String>();
        assert_eq!(padded2, source2);
        assert_eq!(norm2, "Елка");
        assert_eq!(norm2.chars().count(), source2.chars().count());
    }

    #[test]
    fn utf_compose_map_dimenstions() {
        for &(nfd, nfc) in &UTF_COMPOSE_MAP {
            assert_eq!(nfd.chars().count(), 2);
            assert_eq!(nfc.chars().count(), 1);
        }
    }

    #[test]
    fn utf_reduce_map_dimenstions() {
        for &(normal, reduced) in &UTF_REDUCE_MAP {
            assert_eq!(normal .chars().count(), 1, "UTF_REDUCE_MAP['{}'] != 1", normal);
            assert_eq!(reduced.chars().count(), 1, "UTF_REDUCE_MAP['{}'].len() != 1", reduced);
        }
    }
}