use hashbrown::HashMap;
pub fn mirror_lines_with(lines: &[String], map: HashMap<char, char>) -> Vec<String> {
    let mut mirrored_lines: Vec<String> = lines.to_vec();

    // Reverse
    for line in mirrored_lines.iter_mut() {
        *line = line.chars().rev().collect();
    }

    // Replace
    for line in mirrored_lines.iter_mut() {
        *line = line.chars().map(|c| *map.get(&c).unwrap_or(&c)).collect();
    }

    return mirrored_lines;
}

pub fn mirror_lines(lines: &[String]) -> Vec<String> {
    let map: HashMap<char, char> = HashMap::from_iter(
        MIRROR_CHARS
            .into_iter()
            .chain(MIRROR_CHARS.into_iter().map(|(a, b)| (b, a))),
    );
    mirror_lines_with(lines, map)
}

const MIRROR_CHARS: [(char, char); 7] = [
    ('\\', '/'),
    ('\'', '`'),
    ('<', '>'),
    ('p', 'q'),
    ('b', 'd'),
    ('[', ']'),
    ('(', ')'),
];
