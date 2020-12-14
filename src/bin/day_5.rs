use adventofcode2020::*;

fn main() {
    let mut max_seat_id = 0;
    let mut seat_id_list = Vec::new();

    lines_from_file("input/day_5.txt", |line_string| {
        let line = line_string.as_bytes();

        let mut row = 0;
        row += if line[0] == b'B' { 64 } else { 0 };
        row += if line[1] == b'B' { 32 } else { 0 };
        row += if line[2] == b'B' { 16 } else { 0 };
        row += if line[3] == b'B' { 8 } else { 0 };
        row += if line[4] == b'B' { 4 } else { 0 };
        row += if line[5] == b'B' { 2 } else { 0 };
        row += if line[6] == b'B' { 1 } else { 0 };

        let mut col = 0;
        col += if line[7] == b'R' { 4 } else { 0 };
        col += if line[8] == b'R' { 2 } else { 0 };
        col += if line[9] == b'R' { 1 } else { 0 };

        let seat_id = row * 8 + col;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }

        seat_id_list.push(seat_id);
    });

    seat_id_list.sort();

    println!("Part one: {}", max_seat_id);

    for i in 0..(seat_id_list.len() - 1) {
        if seat_id_list[i + 1] - seat_id_list[i] == 2 {
            println!("Part two: {}", seat_id_list[i] + 1);
            break;
        }
    }
}
