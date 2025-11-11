use quest02::ComplexNumber;
use rayon::prelude::*;

fn main() -> std::io::Result<()> {
    let top_left = ComplexNumber::try_from(include_str!("../../notes/part3.txt"))?;

    let divisor = ComplexNumber::new(100000, 100000);

    let points = (0..1001)
        .into_par_iter()
        .flat_map(|y| {
            (0..1001).into_par_iter().map(move |x| {
                let point = top_left + ComplexNumber::new(x, y);
                let mut r = ComplexNumber::new(0, 0);
                let mut escaped = false;
                for _ in 0..100 {
                    r *= r;
                    r /= divisor;
                    r += point;
                    if r.x > 1000000 || r.x < -1000000 || r.y > 1000000 || r.y < -1000000 {
                        escaped = true;
                        break;
                    }
                }
                if !escaped { 1 } else { 0 }
            })
        })
        .sum::<u32>();

    println!("{points}");

    Ok(())
}
