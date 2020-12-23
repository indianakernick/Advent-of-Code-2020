fn main() {
    let mut cups = vec![1, 6, 7, 2, 4, 8, 3, 5, 9];
    //let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let mut current = 0;
    let mut taken_cups = [0, 0, 0];

    for _ in 0..100 {
        let mut destination = cups[current] - 1;
        let mut rotate_by = 3;

        for i in 0..3 {
            let mut taken = current + 1;
            if taken >= cups.len() {
                taken = 0;
                rotate_by -= 1;
            }
            taken_cups[i] = cups.remove(taken);
        }

        while destination < 1 || taken_cups.contains(&destination) {
            destination -= 1;
            if destination < 1 {
                destination = 9;
            }
        }

        for i in 0..cups.len() {
            if cups[i] == destination {
                for j in 0..3 {
                    cups.insert(i + j + 1, taken_cups[j]);
                }
                if i < current {
                    current += rotate_by;
                }
                break;
            }
        }

        current += 1;
        current %= cups.len();
    }

    for i in 0..cups.len() {
        if cups[i] == 1 {
            for j in 0..(cups.len() - 1) {
                print!("{}", cups[(i + j + 1) % cups.len()]);
            }
            println!();
            break;
        }
    }
}