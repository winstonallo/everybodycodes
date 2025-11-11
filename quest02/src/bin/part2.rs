use quest02::ComplexNumber;

fn main() -> std::io::Result<()> {
    let top_left = ComplexNumber::try_from(include_str!("../../notes/part2.txt"))?;
    let mut points = 0;

    for y in 0..101 {
        for x in 0..101 {
            let point = top_left + ComplexNumber::new(x * 10, y * 10);
            let mut r = ComplexNumber::new(0, 0);
            let mut escaped = false;

            for _ in 0..100 {
                r *= r;
                r /= ComplexNumber::new(100000, 100000);
                r += point;
                if r.x > 1000000 || r.x < -1000000 || r.y > 1000000 || r.y < -1000000 {
                    escaped = true;
                    break;
                }
            }

            if !escaped {
                points += 1;
            }
        }
    }

    println!("{points}");

    Ok(())
}
