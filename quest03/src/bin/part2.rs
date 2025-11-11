use quest03::NumberList;

fn main() -> std::io::Result<()> {
    let mut list = NumberList::try_from(include_str!("../../notes/part2.txt"))?;
    list.0.sort();
    list.0.dedup();

    let result = list.0[0..20].iter().sum::<i64>();

    println!("{result}");
    Ok(())
}
