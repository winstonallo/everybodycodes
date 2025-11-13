#[derive(Debug)]
pub struct Notes {
    pub nails: Vec<isize>,
}

impl TryFrom<&str> for Notes {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            nails: value
                .trim()
                .split(",")
                .filter_map(|x| x.parse::<isize>().ok())
                .collect(),
        })
    }
}
