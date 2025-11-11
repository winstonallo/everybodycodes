use quest02::ComplexNumber;

fn main() -> std::io::Result<()> {
    let a = ComplexNumber::try_from(include_str!("../../notes/part1.txt"))?;
    let ten_ten = ComplexNumber::new(10, 10);
    let mut r = ComplexNumber::default();

    for _ in 0..3 {
        r *= r;
        println!("{r}");
        r /= ten_ten;
        println!("{r}");
        r += a;
        println!("{r}");
    }

    println!("{r}");

    Ok(())
}
