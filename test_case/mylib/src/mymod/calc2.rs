pub fn sub(x:i32, y:i32)->i32{
    x-y
}

#[cfg(test)]
mod test_1{
    use super::*;
    #[test]
    fn test_sub(){
        assert_eq!(sub(2,1), 1);
    }
}


