use std::ops::{Add, Mul, Sub};

macro_rules! op {
    ($name:ident, $bound:ident, $method:ident) => {
        fn $name<T>(vx: &mut Vec<T>, vy: &Vec<T>)
        where
            T: $bound<T, Output = T> + Copy,
        {
            for (x, y) in vx.iter_mut().zip(vy.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_test, Add, add);
op!(mul_test, Mul, mul);
op!(sub_test, Sub, sub);

mod my_test {
    use std::iter;
    macro_rules! test {
        ($id:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $id() {
                println!("{} test is running", stringify!($id));
                for i in 0usize..=5 {
                    let mut vx: Vec<_> = iter::repeat($x).take(i).collect();
                    let vy: Vec<_> = iter::repeat($y).take(i).collect();
                    let vz: Vec<_> = iter::repeat($z).take(i).collect();
                    super::$id(&mut vx, &vy);
                    assert_eq!(vx, vz);
                }
            }
        };
    }

    test!(add_test, 1u32, 2u32, 3u32);
    test!(mul_test, 2u32, 3u32, 6u32);
    test!(sub_test, 3u32, 2u32, 1u32);
}
