mod tools;
use std::str;
use itertools::Itertools;


fn main() {
     let input: String = tools::read_file_to_string("everybody_codes_e2024_q1_p3.txt");

     let subs = input.as_bytes()
     .chunks(3)
     .map(str::from_utf8)
     .collect::<Result<Vec<&str>, _>>()
     .unwrap();

     let out1: u32 = subs.into_iter().map(|x|  points(x.to_string())).sum();

     println!("EC 01 p2: {}", out1);

}


fn points(string: String) -> u32{

     let mut x :u32= string.chars().into_iter().map(|x| if x=='B'{1} else if x=='C' {3} else if x=='D' {5} else {0}).sum();
     let v: Vec<_> = string.match_indices("x").collect();
     
     match v.len(){
          0 => x+= 6,
          1 => x+= 2,
          _ => {  }
     }
     
     return  x
}