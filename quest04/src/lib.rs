#[derive(Debug)]
pub struct GearList(pub Vec<i64>);

impl TryFrom<&std::path::Path> for GearList {
    type Error = std::io::Error;

    fn try_from(value: &std::path::Path) -> Result<Self, Self::Error> {
        Ok(Self(
            std::fs::read_to_string(value)?
                .split("\n")
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<i64>>(),
        ))
    }
}

#[derive(Debug)]
pub struct MountedGearList(pub Vec<Gear>);

#[derive(Debug)]
pub enum Gear {
    Single(usize),
    Mounted(usize, usize),
}

impl TryFrom<&std::path::Path> for MountedGearList {
    type Error = std::io::Error;

    fn try_from(value: &std::path::Path) -> Result<Self, Self::Error> {
        Ok(Self(
            std::fs::read_to_string(value)?
                .split("\n")
                .filter_map(|line| match line.split_once("|") {
                    Some((x, y)) => Some(Gear::Mounted(
                        x.parse::<usize>().unwrap(),
                        y.parse::<usize>().unwrap(),
                    )),
                    None => match line.parse::<usize>() {
                        Ok(x) => Some(Gear::Single(x)),
                        Err(_) => None,
                    },
                })
                .collect::<Vec<Gear>>(),
        ))
    }
}
