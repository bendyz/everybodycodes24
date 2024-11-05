mod tools;

fn main() {
     let input: String = tools::read_file_to_string("everybody_codes_e2024_q1_p1.txt");
     // part1
     let out1: i32= input.chars().into_iter().map(|x| if x=='B'{1} else if x=='C' {3} else {0}).sum();
    

     println!("EC 01 p1: {}", out1);

}
