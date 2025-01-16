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
    fn default() -> Monomial {
        Monomial { exponent: 0, coefficients: 0 }
    }
}
struct Polyomial{
    monomials: Vec<Monomial>,
    degree: Option<u32>,    
}

impl Polyomial {
    fn new(monomials: Vec<Monomial>) -> Polyomial {

        Polyomial { monomials, degree: None }
    }
    fn default() -> Polyomial {
        Polyomial { monomials: Vec::new(), degree: None }
    }
    fn evaluate(&self, x: u32) -> u32 {
            let mut result: u32 = 0;     
            let n = self.monomials.len();            
            for i in 0..n{
                result += self.monomials[i].coefficients * x.pow(self.monomials[i].exponent);
            }
            return result;
    }
    fn degree(&mut self) -> Option<u32> {
            let n = self.monomials.len();  
            match self.degree {
                Some(_) => { 
                    let  mut degree = self.degree.unwrap();
                      for i in 0..n{
                if self.monomials[i].exponent > degree {
                    degree = self.monomials[i].exponent;

                }
            }},
                None => self.degree = Some(0),
                
            }     
            return self.degree;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let default = Polyomial::default();
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);   
        let m3 = Monomial::new(0, 5);   

        let p = Polyomial { monomials: vec![m1, m2, m3], ..default };          
        let result = p.evaluate(4);
        assert_eq!(result, 61);
    }

    #[test]
    fn test_degree(){
        let default = Polyomial::default();
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);   

        let mut p = Polyomial { monomials: vec![m1, m2],  ..default  };       
        let result = p.degree().unwrap();
        assert_eq!(result, 2);  
    }
}