use quest03::NumberList;

fn main() -> std::io::Result<()> {
    let list = NumberList::try_from(std::path::Path::new("./notes/part3.txt"))?;
    let mut map = std::collections::HashMap::<i64, usize>::new();
    for n in list.0 {
        *map.entry(n).or_insert(0) += 1;
    }

    let max_freq = map.values().max().unwrap();
    println!("{max_freq}");
    Ok(())
}
