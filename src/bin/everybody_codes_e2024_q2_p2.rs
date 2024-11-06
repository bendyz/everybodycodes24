use std::collections::HashSet;
mod tools;

fn main() {
     let input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q2_p2.txt");
    
    let words = input.get(0).unwrap();

     let mut out1: usize = 0;
     

     for linenum in 2..input.len(){
          
          let mut linev: HashSet<usize> = HashSet::new();
          for word in words.split(","){
               linev.extend(runic_symbol_cnt(word, input.get(linenum).unwrap().clone()))
          }

          out1 += linev.len();
          
     }

     println!("EC 02 p2: {}", out1);


     }
fn runic_symbol_cnt(runic: &str, line : String) -> HashSet<usize>{

     let mut v :HashSet<usize> = HashSet::new();
     let backwards :&str = &runic.chars().rev().collect::<String>();
          for i in 0..=(line.len()-runic.len()){
               if &line[i..i+runic.len()] == runic || &line[i..i+runic.len()] == backwards{
                    for j in i..i+runic.len(){
                         v.insert(j);
                    }
               }

          }

     v
}