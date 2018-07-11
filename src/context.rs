
use num_bigint::BigUint;
use point::Point;
use std::ops::Sub;
use std::ops::Add;
use std::ops::Div;
use std::str::FromStr;
use std::collections::HashMap;
use data_encoding::HEXLOWER;

lazy_static! {
    pub static ref CONTEXT: Context = {
        Context::default()
    };
}

#[allow(non_snake_case)]
pub struct Context {
    pub p: BigUint,
    pub p_sub2: BigUint,
    pub p_sub1_div2: BigUint,
    pub p_add1_div4: BigUint,
    pub two: BigUint,
    pub three: BigUint,
    pub seven: BigUint,
    pub n: BigUint,
    pub G: Point,
}

impl Default for Context {
    fn default() -> Self {
        let p = BigUint::parse_bytes("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F".as_bytes(),16).unwrap();
        let p_sub1 = p.clone().sub(1u32);
        let p_add1 = p.clone().add(1u32);
        Context {
            p : p.clone(),
            p_sub2 : p.clone().sub(2u32),
            p_sub1_div2 : p_sub1.div(2u32),
            p_add1_div4: p_add1.div(4u32),
            two: BigUint::from_str("2").unwrap(),
            three: BigUint::from_str("3").unwrap(),
            seven: BigUint::from_str("7").unwrap(),
            n : BigUint::parse_bytes("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141".as_bytes(),16).unwrap(),
            G : Point {
                x: BigUint::parse_bytes("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798".as_bytes(),16).unwrap(),
                y: BigUint::parse_bytes("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8".as_bytes(),16).unwrap(),
            },
        }
    }
}

lazy_static! {
    pub static ref DOUBLES_CACHE: HashMap<Point, Point> = {
        let mut m = HashMap::new();
        let values = include_str!("cache.in");
        for line in values.lines() {
            let elements : Vec<&str> = line.split(",").collect();
            m.insert(
            Point::from_bytes(&HEXLOWER.decode(elements[0].as_bytes()).unwrap()).unwrap() ,
            Point::from_bytes(&HEXLOWER.decode(elements[1].as_bytes()).unwrap()).unwrap());
        }
        m
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_static() {
        let context = Context::default();
        assert_eq!(CONTEXT.G , context.G);

    }
}