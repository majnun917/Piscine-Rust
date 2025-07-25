#[derive(Debug, PartialEq, Eq)]
pub struct Matrix (pub (i32, i32), pub (i32, i32));

 pub fn transpose(m: Matrix) -> Matrix {
   Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
 }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//struct
//matrix
//types tuple
//Debug, PartialEq and Eq