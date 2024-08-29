use core::ops::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl core::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vector2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl core::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

impl Vector2 {
    pub const ZERO: Self = Self::new(0., 0.);
    pub const ONE: Self = Self::new(1., 1.);
    pub const UNIT_X: Self = Self::new(1., 0.);
    pub const UNIT_Y: Self = Self::new(0., 1.);
    pub const INFINITY: Self = Self::new(f32::INFINITY, f32::INFINITY);
    pub const INFINITY_X: Self = Self::new(f32::INFINITY, 0.);
    pub const INFINITY_Y: Self = Self::new(0., f32::INFINITY);
    pub const NAN: Self = Self::new(f32::NAN, f32::NAN);

    pub const fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    pub fn length_squared(self) -> f32 {
        self * self
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn normalized(self) -> Self {
        let length = self.length();
        if length == 0. {
            Vector2::ZERO
        } else {
            self / self.length()
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul for Vector2 {
    type Output = f32;

    /// Performs the dot product.
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Hitbox {
    pub position: Vector2,
    pub size: Vector2
}

impl Hitbox {
    pub const fn new(width: f32, height: f32, x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height)
        }
    }

    pub fn center(self) -> Vector2 {
        self.position + (self.size / 2.)
    }

    pub fn left(self) -> Vector2 {
        Vector2::new(self.position.x, self.position.y + self.size.y / 2.)
    }

    pub fn right(self) -> Vector2 {
        Vector2::new(self.position.x + self.size.x, self.position.y + self.size.y / 2.)
    }

    pub fn top(self) -> Vector2 {
        Vector2::new(self.position.x + self.size.x / 2., self.position.y)
    }

    pub fn bottom(self) -> Vector2 {
        Vector2::new(self.position.x + self.size.x / 2., self.position.y + self.size.y)
    }
    
    pub fn top_left(self) -> Vector2 {
        Vector2::new(self.position.x, self.position.y)
    }

    pub fn top_right(self) -> Vector2 {
        Vector2::new(self.position.x + self.size.x, self.position.y)
    }

    pub fn is_touching(self, other: Self) -> bool {
        (self.position.x ..= self.position.x + self.size.x).contains(&other.position.x)
            || (self.position.x ..= self.position.x + self.size.x).contains(&(other.position.x + other.size.x))
            || (self.position.y ..= self.position.y + self.size.y).contains(&other.position.y)
            || (self.position.y ..= self.position.y + self.size.y).contains(&(other.position.y + other.size.y))
    }
}