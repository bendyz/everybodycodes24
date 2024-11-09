mod tools;

fn main() {
     let input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q4_p1.txt");
      
     let out1 = "";
     let mut min = input.get(0).unwrap().parse::<usize>().unwrap();

     let mut sum = 0;
     for line in &input{
          let item = line.parse::<usize>().unwrap();
          sum += item;
          if item<min {min = item}
     }

     let out1 = sum - (min * input.len());

     println!("EC 0x pX: {}", out1);


}
