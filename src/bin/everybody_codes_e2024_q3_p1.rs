use std::usize;

mod tools;

fn main() {
     let input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q3_p1.txt");
     // part1
     let out1 = "";
     let my = input.len();
     let mx         = input.get(1).unwrap().len();


     let mut matrix = Matrix::new(mx, my);

     for (linenum,vec) in input.iter().enumerate(){
          let vec1 = vec.chars().collect();
          matrix.add_line(linenum, vec1);
     }


     for i in 2..my{
          matrix.up(i as i16);
     }


     for y in 0..matrix.my{
          for x in 0..matrix.mx{
               print!("{}" ,matrix.get(x,y))
          }
          println!("");
     }
     println!("{}", matrix.get_sum());

     println!("EC 03 p1: {}", out1);


}




#[derive( Copy, Clone,Debug)]
struct Matrix{
     matrix:[[i16;1000];1000],
     mx : usize,
     my : usize
 }


 impl Matrix{
     fn new(mx: usize, my : usize) -> Matrix{
          Matrix{
               matrix: [[0;1000];1000],
               mx: mx,
               my: my
          }
     }

     fn set(&mut self,x: usize, y:usize){
         self.matrix[x][y] += 1; 
     }
     fn get(&mut self,x: usize, y:usize) -> i16{
          self.matrix[x][y]
      }

     fn add_line(&mut self, linenum :usize, vec: Vec<char>){

          for i in 0..self.mx{
               let n = vec.get(i).unwrap();

               let x: i16= match n {
               '#' => 1,
               _ => 0
               };
               self.matrix[i][linenum] = x
          }
     }

     fn get_sum(&self)->i16{

          let mut out:i16 = 0;
          for i in 0..self.mx{
               out += self.matrix[i].iter().map(|&x| x as i16 ).sum::<i16>();
               println!("{out}")
          }

          return out

     }
     
         fn up( &mut self, i: i16){
            
            for x in 1..self.mx-1{
               for y in 1..self.my-1{
                    if  self.get(x, y-1) >= i-1 &&
                        self.get(x-1, y) >= i-1 &&  
                        self.get(x, y+1) >= i-1 &&  
                        self.get(x+1, y) >= i-1{
                         self.set(x, y);
                        }
               }
            }
         }

 }