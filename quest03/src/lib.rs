#[derive(Debug)]
pub struct NumberList(pub Vec<i64>);

impl TryFrom<&str> for NumberList {
    type Error = std::io::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(value)?
            .split(",")
            .filter_map(|x| x.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        Ok(Self(content))
    }
}

impl From<NumberList> for Vec<i64> {
    fn from(value: NumberList) -> Self {
        value.0
    }
}
