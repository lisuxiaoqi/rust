



pub fn add(x:i32, y:i32)->i32{
    x+y
}

pub fn mul(x:i32,y:i32)->i32{
    x * y
}

#[cfg(test)]
mod test_1{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(1,2), 3);
    }
}


#[cfg(test)]
mod test_2{
    extern crate test;
    use test::Bencher;

    use super::*;
    #[test]
    fn test_mul(){
        assert_eq!(mul(1,2), 2)
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add(1,2));
    }
}


