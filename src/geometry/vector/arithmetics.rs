use {
    super::Vector,
    std::ops::{
        Add,
        Mul,
        Sub,
    },
};

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x() + other.x(),
                  self.y() + other.y(),
                  self.z() + other.z())
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x() - other.x(),
                  self.y() - other.y(),
                  self.z() - other.z())
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x() * other.x(),
                  self.y() * other.y(),
                  self.z() * other.z())
    }
}
