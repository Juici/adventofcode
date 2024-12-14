use std::ops::{Add, AddAssign, Sub, SubAssign};

use num::Zero;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T: Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Vec2 { x: T::zero(), y: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
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

impl<A: Sub<B>, B> Sub<Vec2<B>> for Vec2<A> {
    type Output = Vec2<<A as Sub<B>>::Output>;

    fn sub(self, rhs: Vec2<B>) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<A: SubAssign<B>, B> SubAssign<Vec2<B>> for Vec2<A> {
    fn sub_assign(&mut self, rhs: Vec2<B>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
