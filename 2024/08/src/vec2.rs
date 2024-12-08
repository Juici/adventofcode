use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

type Scalar = isize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Scalar> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<Scalar> for Vec2 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<(Scalar, Scalar)> for Vec2 {
    fn from((x, y): (Scalar, Scalar)) -> Self {
        Vec2 { x, y }
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        match (Scalar::try_from(x), Scalar::try_from(y)) {
            (Ok(x), Ok(y)) => Vec2 { x, y },
            _ => panic!("({x}, {y}) is out of the range for Vec2"),
        }
    }
}
