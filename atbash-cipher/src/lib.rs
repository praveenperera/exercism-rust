use std::collections::HashMap;

///"Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let cipher = cipher();
    let char_vec: Vec<char> = plain
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let alphabet: HashMap<char, i32> = alphabet();

    let lookup_vec: Vec<i32> = char_vec
        .into_iter()
        .map(|charchter| *alphabet.get(&charchter).unwrap())
        .collect();

    let result_vec: Vec<char> = lookup_vec
        .into_iter()
        .map(|index| *cipher.get(&index).unwrap())
        .collect();

    result_vec.iter().cloned().collect::<String>()
}

fn cipher() -> HashMap<i32, char> {
    [
        (0, 'z'),
        (1, 'y'),
        (2, 'x'),
        (3, 'w'),
        (4, 'v'),
        (5, 'u'),
        (6, 't'),
        (7, 's'),
        (8, 'r'),
        (9, 'q'),
        (10, 'p'),
        (11, 'o'),
        (12, 'n'),
        (13, 'm'),
        (14, 'l'),
        (15, 'k'),
        (16, 'j'),
        (17, 'i'),
        (18, 'h'),
        (19, 'g'),
        (20, 'f'),
        (21, 'e'),
        (22, 'd'),
        (23, 'c'),
        (24, 'b'),
        (25, 'a'),
    ]
    .iter()
    .cloned()
    .collect()
}

fn alphabet() -> HashMap<char, i32> {
    [
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
        ('q', 16),
        ('r', 17),
        ('s', 18),
        ('t', 19),
        ('u', 20),
        ('v', 21),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('z', 25),
    ]
    .iter()
    .cloned()
    .collect()
}

///'Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
