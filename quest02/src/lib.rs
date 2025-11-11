#[derive(Debug, Clone, Copy)]
pub struct ComplexNumber {
    pub x: i128,
    pub y: i128,
}

impl std::fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Default for ComplexNumber {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl ComplexNumber {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
}

impl TryFrom<&str> for ComplexNumber {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let content = &std::fs::read_to_string(value)?[3..];
        let content = &content[..content.len() - 2]
            .split(",")
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        Ok(Self {
            x: content[0],
            y: content[1],
        })
    }
}

impl std::ops::AddAssign for ComplexNumber {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl std::ops::MulAssign for ComplexNumber {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let real = self.x * rhs.x - self.y * rhs.y;
        self.y = self.x * rhs.y + self.y * rhs.x;
        self.x = real;
    }
}

impl std::ops::DivAssign for ComplexNumber {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl std::ops::Add for ComplexNumber {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self::Output { x, y }
    }
}

impl std::ops::Mul for ComplexNumber {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x - self.y * rhs.y;
        let y = self.x * rhs.y + self.y * rhs.x;
        Self::Output { x, y }
    }
}

impl std::ops::Div for ComplexNumber {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Self::Output { x, y }
    }
}
