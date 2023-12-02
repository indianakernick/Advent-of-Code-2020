pub fn solve(input: &str) -> (u32, u32) {
    let mut sum = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut index = 5;

        let colon = bytes.iter().skip(index).position(|b| *b == b':').unwrap() + index;
        let game_id = std::str::from_utf8(&bytes[index..colon]).unwrap().parse::<u32>().unwrap();

        index = colon + 2;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        while index < bytes.len() {
            let space = bytes.iter().skip(index).position(|b| *b == b' ').unwrap() + index;
            let count = std::str::from_utf8(&bytes[index..space]).unwrap().parse::<u32>().unwrap();

            match bytes[space + 1] {
                b'r' => {
                    max_red = max_red.max(count);
                    index = space + 6;
                }
                b'g' => {
                    max_green = max_green.max(count);
                    index = space + 8;
                }
                b'b' => {
                    max_blue = max_blue.max(count);
                    index = space + 7;
                }
                _ => panic!("Invalid input"),
            }
        }

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += game_id;
        }
    }

    (sum, 0)
}
