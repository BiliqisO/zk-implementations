use std::result;

fn main() {
    println!("Hello, world!");
}
struct Monomial {
    exponent: u32,
    coefficients: u32,
}
impl Monomial
 {
    fn new(exponent: u32, coefficients: u32) -> Monomial {
        Monomial { exponent, coefficients }
    }
}
struct Polyomial{
    monomials: Vec<Monomial>,
    degree: u32,    
}
impl Polyomial {
 fn evaluate(&self, x: u32) -> u32 {
        let mut result: u32 = 0;     
        let n = self.monomials.len();            
        for i in 0..n{
            result += self.monomials[i].coefficients * x.pow(self.monomials[i].exponent);
        }
        return result;
  }
  fn degree(&self) -> u32 {
         let n = self.monomials.len();  
         let mut degree = 0;          
        for i in 0..n{
            if self.monomials[i].exponent > degree {
                degree = self.monomials[i].exponent;
            }

        }
        return degree;

  }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {

        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);   

        let p = Polyomial { monomials: vec![m1, m2], degree: 2 };       
        let result = p.evaluate(4);
        assert_eq!(result, 56);
    }

    #[test]
    fn test_degree(){
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);   

        let p = Polyomial { monomials: vec![m1, m2], degree: 2 };       
        let result = p.degree();
        assert_eq!(result, 2);  
    }
}