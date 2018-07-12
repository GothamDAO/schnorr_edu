use num_bigint::BigUint;
use std::ops::{Sub,Add,Rem,Mul,Div};
use context::CONTEXT;
use super::to_32_bytes;
use super::finite_sub;
use num_traits::One;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ScalarP(pub BigUint);

impl fmt::Display for ScalarP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ScalarP {
    pub fn new(val: BigUint) -> Self {
        match val < CONTEXT.p.0 {
            true  => ScalarP(val),
            false => ScalarP(val.rem(&CONTEXT.p.0)),   // TODO not sure if panic here
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::new(BigUint::from_bytes_be(bytes))
    }
    pub fn to_32_bytes(&self) -> [u8; 32] {
        to_32_bytes(&self.0)
    }
    pub fn pow(&self, n: &ScalarP) -> Self {
        ScalarP(self.0.modpow(&n.0, &CONTEXT.p.0))
    }
    pub fn inv(&self) -> Self {
        ScalarP(self.0.modpow(&CONTEXT.p_sub2.0, &CONTEXT.p.0))
    }

    pub fn is_jacobi(&self) -> bool {
        self.to_owned().pow(&CONTEXT.p_sub1_div2).0.is_one()
    }

}
impl<'a> Sub<&'a ScalarP> for ScalarP {
    type Output = ScalarP;

    fn sub(self, other: &ScalarP) -> ScalarP {
        ScalarP::new(finite_sub(self.0, &other.0, &CONTEXT.p.0))
    }
}
impl<'a> Add<&'a ScalarP> for ScalarP {
    type Output = ScalarP;

    fn add(self, other: &ScalarP) -> ScalarP {
        ScalarP::new(self.0.add(&other.0) )
    }
}
impl<'a> Mul<&'a ScalarP> for ScalarP {
    type Output = ScalarP;

    fn mul(self, other: &ScalarP) -> ScalarP {
        ScalarP::new(self.0.mul(&other.0) )
    }
}
impl<'a> Rem<&'a ScalarP> for ScalarP {
    type Output = ScalarP;

    fn rem(self, other: &ScalarP) -> ScalarP {
        ScalarP(self.0.rem(&other.0) )
    }
}
impl<'a> Div<&'a ScalarP> for ScalarP {
    type Output = ScalarP;

    fn div(self, other: &ScalarP) -> ScalarP {
        ScalarP::new(self.0.div(&other.0) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::CONTEXT;

    #[test]
    fn test_inv() {
        assert!(CONTEXT.G.x.clone().inv().mul(&CONTEXT.G.x).0.is_one());

    }
}