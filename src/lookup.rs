use phf::phf_map;

static LETTERS: phf::Map<&'static str, &'static str> = phf_map! {
    "a" => "Aachen",
    "ä" => "Umlaut Aachen",
    "b" => "Berlin",
    "c" => "Chemnitz",
    "d" => "Düsseldorf",
    "e" => "Essen",
    "f" => "Frankfurt",
    "g" => "Goslar",
    "h" => "Hamburg",
    "i" => "Ingelheim",
    "j" => "Jena",
    "k" => "Köln",
    "l" => "Leipzig",
    "m" => "München",
    "n" => "Nürnberg",
    "o" => "Offenbach",
    "ö" => "Umlaut Offenbach",
    "p" => "Potsdam",
    "q" => "Quickborn",
    "r" => "Rostock",
    "s" => "Salzwedel",
    "ß" => "Eszett",
    "t" => "Tübingen",
    "u" => "Unna",
    "ü" => "Umlaut Unna",
    "v" => "Völklingen",
    "w" => "Wuppertal",
    "x" => "Xanten",
    "y" => "Ypsilon",
    "z" => "Zwickau"
};

pub fn lookup_letter(letter: &str) -> Option<&str> {
    LETTERS.get(letter).map(|v| &**v)
}
