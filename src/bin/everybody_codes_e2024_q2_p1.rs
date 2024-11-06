mod tools;

fn main() {
    let input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q2_p1.txt");
    let words = input.get(0).unwrap();

    let inscription = input.get(2).unwrap();

     let mut out1: usize = 0;
     for word in words.split(","){

          let v: Vec<_> = inscription.match_indices(word).collect();
          out1 += v.len();
     }


     println!("EC 02 p1: {}", out1);


}
