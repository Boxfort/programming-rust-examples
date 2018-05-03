extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {

}

/// Parse the string 's' as a coordinate pair, like '"400x600"' or '"1.0,0.5"'.
///
/// Specifically, 's' should have the form <left><sep><right>, where <sep> is
/// the character given by the 'seperator' argument, and <left> and <right> are both
/// strings that can be parsed by 'T::from_str'.
///
/// If 's' has the proper form, return 'Some<(x,y)>'. If it doesn't parse
/// correctly, return 'None'.
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator){
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])){
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

/// Try to determine if 'c' is in the Mandelbrot set, using at most 'limit'
/// iterations to decide.
///
/// If 'c' is not a member, return 'Some(i)' where 'i' is the number of
/// iterations it took for 'c' to leave the circle of redius two centered on the
/// origin. If 'c' seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that 'c' is not a member),
/// return 'None'.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z : Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
