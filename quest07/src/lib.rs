#[derive(Debug)]
pub struct Rule {
    pub prefix: u8,
    pub possible_suffixes: Vec<u8>,
}

#[derive(Debug)]
pub struct Notes {
    pub names: Vec<String>,
    pub rules: std::collections::HashSet<String>,
}

#[derive(Debug)]
pub struct Notes3 {
    pub names: Vec<String>,
    pub rules: std::collections::HashMap<u8, Vec<u8>>,
}

impl TryFrom<&str> for Rule {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((prefix, possible_suffixes)) =
            value
                .split_once(" > ")
                .and_then(|(prefix_str, suffixes_str)| {
                    let prefix = prefix_str.as_bytes().first().copied()?;
                    let possible_suffixes = suffixes_str
                        .split(',')
                        .filter_map(|x| x.trim().as_bytes().first().copied())
                        .collect::<Vec<u8>>();
                    Some((prefix, possible_suffixes))
                })
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid data",
            ));
        };

        Ok(Self {
            prefix,
            possible_suffixes,
        })
    }
}

impl TryFrom<&str> for Notes {
    type Error = std::io::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split('\n').collect::<Vec<&str>>();

        Ok(Self {
            names: split[0]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
            rules: split[2..]
                .iter()
                .filter(|line| !line.trim().is_empty())
                .fold(
                    std::collections::HashSet::<String>::new(),
                    |mut acc, line| {
                        if let Some((k, v)) = line.split_once(" > ") {
                            v.split(",").for_each(|s| {
                                acc.insert(k.to_string() + s);
                            });
                        }
                        acc
                    },
                ),
        })
    }
}

impl TryFrom<&str> for Notes3 {
    type Error = std::io::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split('\n').collect::<Vec<&str>>();

        Ok(Self {
            names: split[0]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
            rules: split[2..]
                .iter()
                .filter(|line| !line.trim().is_empty())
                .fold(
                    std::collections::HashMap::<u8, Vec<u8>>::new(),
                    |mut acc, line| {
                        if let Some((k, v)) = line.split_once(" > ") {
                            let key = k.as_bytes()[0];
                            let values = v
                                .split(",")
                                .map(|val| val.as_bytes()[0])
                                .collect::<Vec<u8>>();
                            acc.insert(key, values);
                        }
                        acc
                    },
                ),
        })
    }
}
