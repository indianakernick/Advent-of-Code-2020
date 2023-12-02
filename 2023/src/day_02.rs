pub fn solve(input: &str) -> (u32, u32) {
    let mut id_sum = 0;
    let mut power_sum = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut index = 5; // skip to game ID

        let colon = index_of(bytes, index, b':');
        let game_id = parse(&bytes[index..colon]);

        index = colon + 2; // skip to first count

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        while index < bytes.len() {
            let space = index_of(bytes, index, b' ');
            let count = parse(&bytes[index..space]);

            let (max, length) = match bytes[space + 1] {
                b'r' => (&mut max_red, 6),
                b'g' => (&mut max_green, 8),
                b'b' => (&mut max_blue, 7),
                _ => panic!("Invalid input"),
            };

            *max = (*max).max(count);
            index = space + length; // skip past this count to next count
        }

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            id_sum += game_id;
        }

        power_sum += max_red * max_green * max_blue;
    }

    (id_sum, power_sum)
}

fn index_of(bytes: &[u8], index: usize, needle: u8) -> usize {
    bytes[index..]
        .iter()
        .position(|b| *b == needle)
        .unwrap()
        + index
}

fn parse(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b - b'0') as u32 * 10u32.pow(i as u32))
        .sum()
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let output = solve(input);
    assert_eq!(output.0, 8);
    assert_eq!(output.1, 2286);
}
