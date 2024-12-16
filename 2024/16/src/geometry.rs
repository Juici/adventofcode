use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use num::{Integer, One, Signed, Zero};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn vector<T: One + Zero + Neg<Output = T>>(self) -> Vec2<T> {
        let one = T::one();
        let zero = T::zero();

        let (x, y) = match self {
            Direction::North => (zero, -one),
            Direction::East => (one, zero),
            Direction::South => (zero, one),
            Direction::West => (-one, zero),
        };

        Vec2 { x, y }
    }

    pub fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T: Signed + Integer + Clone> Vec2<T> {
    pub fn adjacent(&self, dir: Direction) -> Vec2<T> {
        dir.vector::<T>() + self
    }
}

impl<T: Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Vec2 { x: T::zero(), y: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<A: Add<B>, B: Clone> Add<&Vec2<B>> for Vec2<A> {
    type Output = Vec2<<A as Add<B>>::Output>;

    fn add(self, rhs: &Vec2<B>) -> Self::Output {
        Vec2 { x: self.x + rhs.x.clone(), y: self.y + rhs.y.clone() }
    }
}

impl<A: Add<B>, B> Add<Vec2<B>> for Vec2<A> {
    type Output = Vec2<<A as Add<B>>::Output>;

    fn add(self, rhs: Vec2<B>) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<A: AddAssign<B>, B> AddAssign<Vec2<B>> for Vec2<A> {
    fn add_assign(&mut self, rhs: Vec2<B>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<A: AddAssign<B>, B: Clone> AddAssign<&Vec2<B>> for Vec2<A> {
    fn add_assign(&mut self, rhs: &Vec2<B>) {
        self.x += rhs.x.clone();
        self.y += rhs.y.clone();
    }
}

impl<A: Sub<B>, B> Sub<Vec2<B>> for Vec2<A> {
    type Output = Vec2<<A as Sub<B>>::Output>;

    fn sub(self, rhs: Vec2<B>) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<A: Sub<B>, B: Clone> Sub<&Vec2<B>> for Vec2<A> {
    type Output = Vec2<<A as Sub<B>>::Output>;

    fn sub(self, rhs: &Vec2<B>) -> Self::Output {
        Vec2 { x: self.x - rhs.x.clone(), y: self.y - rhs.y.clone() }
    }
}

impl<A: SubAssign<B>, B> SubAssign<Vec2<B>> for Vec2<A> {
    fn sub_assign(&mut self, rhs: Vec2<B>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<A: SubAssign<B>, B: Clone> SubAssign<&Vec2<B>> for Vec2<A> {
    fn sub_assign(&mut self, rhs: &Vec2<B>) {
        self.x -= rhs.x.clone();
        self.y -= rhs.y.clone();
    }
}

impl<A: Mul<B>, B: Clone> Mul<B> for Vec2<A> {
    type Output = Vec2<<A as Mul<B>>::Output>;

    fn mul(self, rhs: B) -> Self::Output {
        Vec2 { x: self.x * rhs.clone(), y: self.y * rhs }
    }
}
