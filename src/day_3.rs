fn split_components(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn char_score(c: char) -> usize {
    match c {
        'a'..='z' => 1 + (c as u8 - b'a') as usize,
        'A'..='Z' => 27 + (c as u8 - b'A') as usize,
        _ => 0,
    }
}

fn find_common_char(strs: (&str, &str)) -> Option<char> {
    strs.0.chars().find(|c| strs.1.contains(*c))
}

pub fn puzzle_1(data: String) {
    let sum_prios = data
        .lines()
        .map(&split_components)
        .map(&find_common_char)
        .map(|c| c.map_or(0, &char_score))
        .sum::<usize>();
    println!("Total priority of shared items is {}", sum_prios);
}

pub fn puzzle_2(data: String) {
    let lines = data.lines().collect::<Vec<_>>();
    let sum_prios = lines
        .chunks_exact(3)
        .map(|set| {
            set[0]
                .chars()
                .find(|b| set[1].contains(*b) && set[2].contains(*b))
        })
        .map(|c| c.map_or(0, &char_score))
        .sum::<usize>();
    println!(
        "Total priority of badges in each 3 elf group is {}",
        sum_prios
    );
}
