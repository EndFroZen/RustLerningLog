
struct Solution;

impl Solution {
    fn convert_number_to_vector(number:i32) -> Vec<char>{
        let num_string_convert: String= number.to_string();
        let char_number_string: std::str::Chars<'_> = num_string_convert.chars();
        let vec_number_string:Vec<char> = char_number_string.collect();
        return vec_number_string;
        
    }
    pub fn is_palindrome(x: i32) -> bool {
        let vec_number_conver: Vec<char> = Solution::convert_number_to_vector(x);
        let mut vec_number_conver_revers: Vec<char> = Solution::convert_number_to_vector(x);
        vec_number_conver_revers.reverse();
        
        println!("1. {:?}",vec_number_conver);
        println!("2. {:?}",vec_number_conver_revers);
        return vec_number_conver_revers==vec_number_conver;
    }
}


fn main() {
    let input: i32 = 121;
    let output = Solution::is_palindrome(input);
    print!("{}",output)
}
