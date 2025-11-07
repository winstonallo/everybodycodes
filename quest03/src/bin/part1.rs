use quest03::NumberList;

fn main() -> std::io::Result<()> {
    let mut list = NumberList::try_from(std::path::Path::new("./notes/part1.txt"))?;
    list.0.sort();
    list.0.dedup();

    let result = list.0.iter().sum::<i64>();

    println!("{result}");
    Ok(())
}
