use std::ops::{Add, AddAssign, Sub, SubAssign};

use num::rational::Ratio;
use num::Zero;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    // Gradient.
    a: Ratio<i64>,
    // Y-intercept.
    c: Ratio<i64>,
}

impl Line {
    pub fn new(gradient: impl Into<Ratio<i64>>, y_intercept: impl Into<Ratio<i64>>) -> Line {
        Line { a: gradient.into(), c: y_intercept.into() }
    }

    /// Creates a line from the parameters of the equation: ax + by = c
    pub fn from_abc(
        a: impl Into<Ratio<i64>>,
        b: impl Into<Ratio<i64>>,
        c: impl Into<Ratio<i64>>,
    ) -> Line {
        let a = a.into();
        let b = b.into();
        let c = c.into();

        Line::new(-a / b, c / b)
    }

    pub fn intersection(&self, other: &Line) -> Option<Vec2<Ratio<i64>>> {
        let Line { a, c } = self;
        let Line { a: b, c: d } = other;

        let n = d - c;
        let m = a - b;

        // Check if lines are parallel.
        if n.is_zero() {
            // Check if lines are coincident.
            return if m.is_zero() { Some(Vec2::zero()) } else { None };
        }

        let x = n / m;
        let y = a * x + c;

        Some(Vec2 { x, y })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2<T = i64> {
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
