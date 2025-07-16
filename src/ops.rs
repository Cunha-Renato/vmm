pub trait ScalarOps {
    type Number;

    fn add_assign_scalar(&mut self, scalar: Self::Number);
    fn sub_assign_scalar(&mut self, scalar: Self::Number);
    fn mul_assign_scalar(&mut self, scalar: Self::Number);
    fn div_assign_scalar(&mut self, scalar: Self::Number);
}

pub trait VectorOps: ScalarOps {
    type Result;

    fn dot(&self, rhs: &Self) -> Self::Result;
    fn cross(&self, rhs: &Self);
    fn add_assign(&mut self, rhs: &Self);
    fn sub_assign(&mut self, rhs: &Self);
}

pub trait MatrixOps: ScalarOps {
    fn transpose(&self) -> Self;
    fn transpose_assign(&mut self);
    
    fn add_assign(&mut self, rhs: &Self);
    fn sub_assign(&mut self, rhs: &Self);
    fn mul_assign(&mut self, rhs: &Self);
}

impl<T> ScalarOps for [T]
where
    T: Clone + num_traits::NumOps + num_traits::NumAssignOps,
{
    type Number = T;

    #[inline(always)]
    fn add_assign_scalar(&mut self, scalar: Self::Number) {
        self.iter_mut().for_each(|x| *x += scalar.clone());
    }

    #[inline(always)]
    fn sub_assign_scalar(&mut self, scalar: Self::Number) {
        self.iter_mut().for_each(|x| *x -= scalar.clone());
    }

    #[inline(always)]
    fn mul_assign_scalar(&mut self, scalar: Self::Number) {
        self.iter_mut().for_each(|x| *x *= scalar.clone());
    }

    #[inline(always)]
    fn div_assign_scalar(&mut self, scalar: Self::Number) {
        self.iter_mut().for_each(|x| *x /= scalar.clone());
    }
}
impl<T> VectorOps for [T]
where
    T: Clone + num_traits::NumOps + num_traits::NumAssignOps + std::iter::Sum,
{
    type Result = T;

    #[inline(always)]
    fn dot(&self, rhs: &Self) -> Self::Result {
        assert_eq!(self.len(), rhs.len());
        self.iter()
            .zip(rhs)
            .map(|(a, b)| a.clone() * b.clone())
            .sum()
    }
    
    #[inline(always)]
    fn cross(&self, rhs: &Self) {
        todo!()
    }
    
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(self.len(), rhs.len());
        self.iter_mut()
            .zip(rhs)
            .for_each(|(a, b)| *a += b.clone());
    }

    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        assert_eq!(self.len(), rhs.len());
        self.iter_mut()
            .zip(rhs)
            .for_each(|(a, b)| *a -= b.clone());
    }
}
