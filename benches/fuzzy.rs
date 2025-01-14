use {
    broot::pattern::FuzzyPattern,
    criterion::{black_box, criterion_group, criterion_main, Criterion},
};

static PATTERNS: &[&str] = &["réveil", "AB", "e", "brt", "brootz"];
static NAMES: &[&str] = &[
    " brr ooT",
    "Réveillon",
    "dys",
    "test",
    " tetsesstteststt ",
    "a rbrroot",
    "Ab",
    "test again",
    "des réveils",
    "pi",
    "a quite longuer name",
    "compliqué - 这个大象有多重",
    "brrooT",
    "1",
    "another name.jpeg",
    "aaaaaab",
    "a ab abba aab",
    "abcdrtodota",
    "palimpsestes désordonnés",
    "a",
    "π",
    "normal.dot",
    "ùmeé9$njfbaù rz&é",
    "FactoryFactoryFactoryFactory.java",
    "leftPad.js",
    "Cargo.toml",
    "Cargo.lock",
    "main.rs",
    ".gitignore",
    "lib.rs",
    " un réveil",
    "aaaaaaaaaaaaaaaaabbbbbbb",
    "BABABC B AB",
    "réveils",
    "paem",
    "poëme",
    "mjrzemrjzm mrjz mrzr rb root",
    "&cq",
    "..a",
    "~~~~~",
    "ba",
    "bar",
    "bar ro ot",
    "& aé &a é",
    "mùrz*jfzùenfzeùrjmùe",
    "krz",
    "q",
    "mjrfzm e",
    "dystroy.org",
    "www",
    "termimad",
    "minimad",
    "regex",
    "lazy_regex",
    "jaquerie",
    "Tillon",
    "Tellini",
    "Garo",
    "Portequoi",
    "Terdi",
    "Ploplo",
    "le dragon",
    "l'ours",
    "la tortue géante",
    "le chamois",
    "dystroy",
    "bra ernre rjrz a e3 broorar/ e/ smallvec/memmap;r b oot4 Z",
    "un petit peu n'importe quoi",
    "dans",
    "cette",
    "liste",
    "Broot",
    " broot",
    " broot ",
    "b-root",
    "biroute",
    "Miaou",
    "meow",
    "et",
    "surtout",
    "La Grande Roulette",
    "this list is",
    "very obviously",
    "tailored at stressing",
    "the engine",
    "and the reader",
    "C++",
    "javascript",
    "SQL",
    "C#",
    "Haskell",
    "Lisp",
    "Pascal",
    "and",
    "Fortran",
    "are just missing from this codebase",
    "denys",
    "seguret",
    "is",
    "the",
    "author",
    "bro o o o o  o o o  o o o o o ot",
    "bro o o o o  o o o  o o o o o otz",
    "br bro boo broot brootz",
    "b b bb bb ca e 1234 oooot",
    "Bo br BBBroo OOOOOt",
    "kir ba lrbvr b rbaz broot",
    "nrel ora hr rbooo t roo jrzz 7 tz",
    "not matching anything, is it ?",
    "ae/r/re /reee/ea",
    "era",
    "lrlb rre o",
    "rjre nr",
];

fn score_of_benchmark(c: &mut Criterion) {
    for pattern in PATTERNS {
        let task = format!("FuzzyPattern({:?})::score_of", pattern);
        c.bench_function(&task, |b| {
            let fp = FuzzyPattern::from(pattern);
            b.iter(|| {
                for name in NAMES {
                    black_box(fp.score_of(name));
                }
            });
        });
        let task = format!("FuzzyPattern({:?})::find", pattern);
        c.bench_function(&task, |b| {
            let fp = FuzzyPattern::from(pattern);
            b.iter(|| {
                for name in NAMES {
                    black_box(fp.find(name));
                }
            });
        });
    }
}

criterion_group!(benches, score_of_benchmark);
criterion_main!(benches);
