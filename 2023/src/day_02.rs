use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut id_sum = 0;
    let mut power_sum = 0;

    for bytes in common::lines_iter(input) {
        let mut index = 5; // skip to game ID

        let colon = common::index_of_after(bytes, b':', index);
        let game_id = common::parse_u32(&bytes[index..colon]);

        index = colon + 2; // skip to first count

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        while index < bytes.len() {
            let space = common::index_of_after(bytes, b' ', index);
            let count = common::parse_u32(&bytes[index..space]);

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
