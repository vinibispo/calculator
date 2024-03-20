use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Debug, Clone, Copy)]
pub enum InterpreterType {
    Integer(i32),
    Real(f64),
}

impl Neg for InterpreterType {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            InterpreterType::Integer(value) => InterpreterType::Integer(-value),
            InterpreterType::Real(value) => InterpreterType::Real(-value),
        }
    }
}

impl Add for InterpreterType {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (InterpreterType::Integer(left), InterpreterType::Integer(right)) => {
                InterpreterType::Integer(left + right)
            }
            (InterpreterType::Real(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left + right)
            }
            (InterpreterType::Integer(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left as f64 + right)
            }
            (InterpreterType::Real(left), InterpreterType::Integer(right)) => {
                InterpreterType::Real(left + right as f64)
            }
        }
    }
}

impl Sub for InterpreterType {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (InterpreterType::Integer(left), InterpreterType::Integer(right)) => {
                InterpreterType::Integer(left - right)
            }
            (InterpreterType::Real(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left - right)
            }
            (InterpreterType::Integer(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left as f64 - right)
            }
            (InterpreterType::Real(left), InterpreterType::Integer(right)) => {
                InterpreterType::Real(left - right as f64)
            }
        }
    }
}

impl Mul for InterpreterType {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (InterpreterType::Integer(left), InterpreterType::Integer(right)) => {
                InterpreterType::Integer(left * right)
            }
            (InterpreterType::Real(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left * right)
            }
            (InterpreterType::Integer(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left as f64 * right)
            }
            (InterpreterType::Real(left), InterpreterType::Integer(right)) => {
                InterpreterType::Real(left * right as f64)
            }
        }
    }
}

impl Div for InterpreterType {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (InterpreterType::Integer(left), InterpreterType::Integer(right)) => {
                InterpreterType::Real(left as f64 / right as f64)
            }
            (InterpreterType::Real(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left / right)
            }
            (InterpreterType::Integer(left), InterpreterType::Real(right)) => {
                InterpreterType::Real(left as f64 / right)
            }
            (InterpreterType::Real(left), InterpreterType::Integer(right)) => {
                InterpreterType::Real(left / right as f64)
            }
        }
    }
}

impl From<InterpreterType> for f64 {
    fn from(value: InterpreterType) -> f64 {
        match value {
            InterpreterType::Integer(value) => value as f64,
            InterpreterType::Real(value) => value,
        }
    }
}

impl From<InterpreterType> for i32 {
    fn from(value: InterpreterType) -> i32 {
        match value {
            InterpreterType::Integer(value) => value,
            InterpreterType::Real(value) => value as i32,
        }
    }
}

impl InterpreterType {
    pub fn from<T>(&self) -> T
    where
        T: From<InterpreterType>,
    {
        T::from(*self)
    }
    pub fn integer_div(self, other: Self) -> Self {
        match (self, other) {
            (InterpreterType::Integer(left), InterpreterType::Integer(right)) => {
                InterpreterType::Integer(left / right)
            }
            (InterpreterType::Real(left), InterpreterType::Real(right)) => {
                InterpreterType::Integer((left as i32 / right as i32) as i32)
            }
            (InterpreterType::Integer(left), InterpreterType::Real(right)) => {
                InterpreterType::Integer((left / right as i32) as i32)
            }
            (InterpreterType::Real(left), InterpreterType::Integer(right)) => {
                InterpreterType::Integer((left as i32 / right) as i32)
            }
        }
    }
}
