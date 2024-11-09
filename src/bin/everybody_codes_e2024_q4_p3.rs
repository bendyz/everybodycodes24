mod tools;

fn main() {
     let input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q4_p3.txt");
      

     let sumall : u64= input.iter().map(|x| x.parse::<u64>().unwrap()).sum();

    let mid:f64 = (sumall as f64 / (f64::from(input.len() as i32)).round()) as f64;
    let midi64 = f64::round(mid) as i64;
     // println!("mid: {mid} suma: {sumall} ilosc rekordow: {}",input.len());
     // let mut sum: usize = 0;
     // for line in &input{
     //      let item = line.parse::<i64>().unwrap();
     //      sum += i64::abs((item  - mid )) as usize;
     // }
     // println!("EC 04 p2: {}", sum);
     
     dbg!(mid);
     dbg!(midi64);

     let mut bestmid = 0;
     let mut  min:i64 = 4343;
      for mid in 1..22{
        //  let mid:i64 = sumall/i;
      //    println!("mid: {mid} suma: {sumall} ilosc rekordow: {}",input.len());
          let mut sum: i64 = 0;
          for line in &input{
               let item = line.parse::<i64>().unwrap();
               sum += i64::abs((item  - mid )) as i64;
          }
          println!("mid: {mid} suma: {sum} ");
          if min > sum { min = sum; bestmid = mid};
      }
      println!("best mid: {bestmid}");
     // let mid:i64 = 27480684;
     // println!("mid: {mid} suma: {sumall} ilosc rekordow: {}",input.len());
     // let mut sum: usize = 0;
     // for line in &input{
     //      let item = line.parse::<i64>().unwrap();
     //      sum += i64::abs((item  - mid )) as usize;
     // }
     println!("EC 04 p2: {}", min);


     // let mid:i32 = 27480685;
     // println!("mid: {mid} suma: {sumall} ilosc rekordow: {}",input.len());
     // let mut sum: usize = 0;
     // for line in &input{
     //      let item = line.parse::<i32>().unwrap();
     //      sum += i32::abs((item  - mid )) as usize;
     // }
     // println!("EC 04 p2: {}", sum);

//prawidlowy wynik = 120958468
}
