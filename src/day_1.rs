pub fn puzzle_1(data: String) {
    let lines = data
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let result = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|value| value.unwrap()).sum::<usize>())
        .max()
        .unwrap();
    println!("Top elf has {} calories", result);
}

pub fn puzzle_2( data: String )
{
    let lines = data
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let mut values = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|value| value.unwrap()).sum::<usize>())
        .collect::<Vec<_>>();
    values.sort_by_key(|value|std::cmp::Reverse(*value));
    let result = values.iter().take(3).sum::<usize>();
    println!( "Top 3 elves have {} calories", result );
}
