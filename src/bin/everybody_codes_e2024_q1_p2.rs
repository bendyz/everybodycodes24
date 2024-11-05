mod tools;
use std::str;
use itertools::Itertools;


fn main() {
     let input: String = tools::read_file_to_string("everybody_codes_e2024_q1_p2.txt");

     let subs = input.as_bytes()
     .chunks(2)
     .map(str::from_utf8)
     .collect::<Result<Vec<&str>, _>>()
     .unwrap();

     let out1: u32 = subs.into_iter().map(|x|  match x {
          "AA" => 2,
          "AB" => 3,
          "AC" => 5,
          "AD" => 7,
          
          "BA" => 3,  
          "BB" => 4,
          "BC" => 6,
          "BD" => 8,
          "Bx" => 1,

          "CA" => 5,
          "CB" => 6,
          "CC" => 8,
          "CD" => 10,
          "Cx" => 3,

          "DA" => 7,
          "DB" => 8,
          "DC" => 10,
          "DD" => 12,
          "Dx" => 5,      

          "xB" => 1,
          "xC" => 3,
          "xD" => 5,
          _ => 0
     }).sum();

     println!("EC 01 p2: {}", out1);

}
