use quest07::Notes3 as Notes;

fn solve(
    name: &Vec<u8>,
    rules: &std::collections::HashMap<u8, Vec<u8>>,
    unique_names: &mut std::collections::HashSet<Vec<u8>>,
) {
    if name.len() > 11 {
        return;
    }

    if name.len() >= 7 {
        unique_names.insert(name.clone());
    }

    if let Some(suffixes) = rules.get(&name[name.len() - 1]) {
        for suffix in suffixes {
            let mut new_name = name.clone();
            new_name.push(*suffix);
            solve(&new_name, rules, unique_names);
        }
    }
}

fn is_valid_prefix(name: &Vec<u8>, rules: &std::collections::HashMap<u8, Vec<u8>>) -> bool {
    for i in 0..name.len() - 1 {
        if let Some(allowed) = rules.get(&name[i]) {
            if !allowed.contains(&name[i + 1]) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part3.txt"))?;
    let mut unique_names = std::collections::HashSet::new();

    for name in notes.names {
        let name_bytes = name.as_bytes().to_vec();
        if is_valid_prefix(&name_bytes, &notes.rules) {
            solve(&name_bytes, &notes.rules, &mut unique_names);
        }
    }

    println!("{}", unique_names.len());
    Ok(())
}
