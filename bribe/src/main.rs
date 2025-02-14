fn minimum_bribes(q: &[i32]) {
    let mut bribes = 0;

    for (i, &person) in q.iter().enumerate() {
        let original_pos = person as usize - 1;

        if original_pos > i + 2 {
            println!("Too chaotic");
            return;
        }

        for j in (original_pos.saturating_sub(2))..i {
            if q[j] > person {
                println!("original: {}", original_pos);
                println!("o: {}...{}", original_pos.saturating_sub(2), i);
                println!("i: {}", i);
                println!("j: {}", j);

                bribes += 1;
            }
        }
    }

    println!("{}", bribes);
}

fn main() {
    let q = [1, 4, 2, 3];
    minimum_bribes(&q);
}
