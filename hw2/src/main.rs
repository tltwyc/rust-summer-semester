// Rust summer semester: Hw2
// tltwyc
// Ex: 11/17/26 in slides

fn main() {
    println!("please use cargo test");
}

// Ex 1 -----------------------------------
struct Buffer<T> {
    pub data: Vec<T>
}

impl<T: Copy + std::iter::Sum<T>> Buffer<T> {
    pub fn new(data: Vec<T>) -> Self {
        Buffer::<T> {
            data
        }
    }
    pub fn sum(&self) -> T {
        let iter = self.data.clone().into_iter();
        iter.sum()
    }
}

// Ex 2 ------------------------------------
fn compare_string(x: &str, y: &str) -> bool {
    let xp: Vec<char> = x.chars().collect();
    let yp: Vec<char> = y.chars().collect();
    if xp.len() == yp.len(){
        for i in 0..xp.len(){
            if xp.get(i) > yp.get(i) {
                return true;
            } else if xp.get(i) < yp.get(i) {
                return false;
            }
        }
    }
    xp.len() > yp.len()
}

// Ex 3 -------------------------------------
fn map_add_1(v: &Vec<char>) -> Vec<char>{
    v.iter().map(|&c| (c as u8 + 1) as char).collect()
}


// test -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex1(){
        let v1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let buf = Buffer::new(v1.to_vec());
        assert_eq!(buf.sum(), 45);

        let v2 = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9];
        let buf = Buffer::new(v2.to_vec());
        assert_eq!(buf.sum(), 5.5*9.0);
    }

    #[test]
    fn test_ex2(){
        let str1 = "abcde";
        let str2 = "lmnop";
        let str3 = "lmnoz";
        let str4 = "lmn";
        let str5 = "lmnopq";
        assert_eq!(compare_string(str1, str1), false);
        assert_eq!(compare_string(str1, str2), false);
        assert_eq!(compare_string(str3, str2), true);
        assert_eq!(compare_string(str2, str4), true);
        assert_eq!(compare_string(str2, str5), false);
    }

    #[test]
    fn test_ex3(){
        let v: Vec<char> = ['a', 'b', 'c', 'd', 'e'].to_vec();
        assert_eq!(map_add_1(&v), ['b', 'c', 'd', 'e', 'f'].to_vec());
    }
}