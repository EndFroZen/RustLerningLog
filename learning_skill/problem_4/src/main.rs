struct Solution;

impl Solution {
    fn sting_to_char(input_sting: String)->Vec<char>{
        let string_is_char: std::str::Chars<'_> = input_sting.chars();
        let char_to_vec:Vec<char>=string_is_char.collect();
        return char_to_vec ;
    }
    pub fn roman_to_int(s: String) -> i32 {
        //1.แปลง sting ให้เป็น Vec<char> ให้พร้อมใช้งาน
        let s_to_char: Vec<char>= Solution::sting_to_char(s);
        println!("{:?}",s_to_char);
        return 0;
    }
}

fn main(){
    let roman_number_input:String = String::from("III");
    let conver_roman_output: i32 = Solution::roman_to_int(roman_number_input);
    println!("{}",conver_roman_output)

}