use std::ops::{Add, Sub, Mul, Neg};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VecN<T, const N: usize> {
    pub components: [T; N],
}

impl<T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd, const N: usize> Index<usize> for VecN<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl<T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd, const N: usize> IndexMut<usize> for VecN<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl<T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + PartialOrd + Into<f32> + From<f32>, const N: usize> VecN<T, N> 
{

    pub fn normalize(&self) -> Self {
        let len = self.len();
        let mut result = *self;
        if len != 0.0 {
            for i in 0..N {
                result.components[i] = T::from(result.components[i].into() / len);
            }
        }
        result
    }
}

impl<T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + PartialOrd + Into<f32>, const N: usize> VecN<T, N> 
{
    pub fn new(value: T) -> Self 
	{
        Self {
            components: [value; N],
        }
    }

    pub fn len(&self) -> f32 
	{
        self.components.iter().map(|&x| (x.into() * x.into())).sum::<f32>().sqrt()
    }

    pub fn distance(&self, other: &Self) -> f32 
	{
        (*self - *other).len()
    }

    pub fn dot(&self, rhs: &Self) -> T 
	{
        let mut result = T::default();
        for i in 0..N 
		{
            result = result + self.components[i] * rhs.components[i];
        }
        result
    }

    // Negate function
    pub fn negate(&self) -> Self 
	{
        let mut result = *self;
        for i in 0..N 
		{
            result.components[i] = -result.components[i];
        }
        result
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>, const N: usize> Add for VecN<T, N> 
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output 
	{
        let mut result = self;
        for i in 0..N 
		{
            result.components[i] = self.components[i] + rhs.components[i];
        }
        result
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>, const N: usize> Sub for VecN<T, N> 
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output 
	{
        let mut result = self;
        for i in 0..N 
		{
            result.components[i] = self.components[i] - rhs.components[i];
        }
        result
    }
}

impl<T: Copy + Mul<Output = T>, const N: usize> Mul<T> for VecN<T, N> 
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output 
	{
        let mut result = self;
        for i in 0..N 
		{
            result.components[i] = self.components[i] * rhs;
        }
        result
    }
}