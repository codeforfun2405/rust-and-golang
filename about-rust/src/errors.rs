use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MyError {
    #[error("invalid num")]
    InvalidNum,
}

pub fn sum_data(data: Vec<i32>) -> Result<i32, MyError> {
    let mut sum = 0;
    for num in data {
        if num < 0 {
            return Err(MyError::InvalidNum);
        }
        sum += num;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::errors::MyError;

    use super::sum_data;

    #[test]
    fn test_sum_data() {
        let sum_result = sum_data(vec![1, 2, 3]);
        assert_eq!(true, sum_result.is_ok());
        assert_eq!(6, sum_result.ok().unwrap());

        let sum_result = sum_data(vec![1, -2, 3]);
        assert_eq!(true, sum_result.is_err());
        assert_eq!(MyError::InvalidNum, sum_result.err().unwrap());
    }
}
