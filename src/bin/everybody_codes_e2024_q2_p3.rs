use std::collections::HashSet;
mod tools;

fn main() {
     let mut input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q2_p3.txt");
    
    let words = input.get(0).unwrap().clone();
    let mut out1 : usize = 0;

     input.remove(0);
     input.remove(0);
     

     let mut linev: HashSet<Point> = HashSet::new();
     for word in words.split(","){
          linev.extend(runic_symbol_cnt(word, &input))
     }

     out1 += linev.len();
          
     println!("EC 02 p3: {}", out1);


     }
fn runic_symbol_cnt(runic: &str, lines : &Vec<String>) -> HashSet<Point>{

     let mut v :HashSet<Point> = HashSet::new();
     let backwards :&str = &runic.chars().rev().collect::<String>();


     for y in 0..lines.len(){

          //horizontal
          let mut line = lines.get(y).unwrap().clone();
          let line_len = line.len();
          line.push_str(&line.clone());
          for i in 0..line_len{
               let mut string_v : String = "".to_string();
               let mut v_temp :HashSet<Point> = HashSet::new();
               for g in i..i+runic.len(){
                    let x1: usize = if g< line_len{
                         g
                      } else {
                         g - line_len
                         };
                    string_v.push(line.chars().nth(g).unwrap()); 
                    v_temp.insert(Point{x:x1,y:y});
               }

               if string_v == runic || string_v ==backwards{
                    v.extend(v_temp);
                    
               }

          }

          //vertical
          if y as i32<=(lines.len() as i32 - runic.len() as i32){
          for m in 0..line_len{
               let mut string_v : String = "".to_string();
               let mut v_temp :HashSet<Point> = HashSet::new();
               for y1 in y..y+runic.len(){
                    string_v.push(lines.get(y1).unwrap().chars().nth(m).unwrap()); 
                    v_temp.insert(Point{x:m,y:y1});
               }

               if string_v == runic || string_v ==backwards{
                    v.extend(v_temp);
               
               }
          }
          }
     }
 v
}

#[derive(PartialEq, Eq, Hash,Debug,Clone)]
struct Point{
     x:usize,
     y:usize
}