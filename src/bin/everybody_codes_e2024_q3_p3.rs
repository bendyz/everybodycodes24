use std::usize;

mod tools;

fn main() {
    let input: Vec<String> = tools::read_file_to_lines("everybody_codes_e2024_q3_p3.txt");
    // part1
    let out1 = "";

    let my = input.len();
    let mx = input.get(1).unwrap().len();

    let mut matrix = Matrix::new(mx, my);

    for (linenum, vec) in input.iter().enumerate() {
        let vec1 = vec.chars().collect();
        matrix.add_line(linenum, vec1);
    }

    for i in 2..13 {
        matrix.up(i as i16);
    }

    for y in 0..matrix.my {
        for x in 0..matrix.mx {
            print!("{}", matrix.get(x as i16, y as i16))
        }
        println!("");
    }

    matrix.check();
    println!("{}", matrix.get_sum());

    println!("EC 03 p1: {}", out1);
}

#[derive(Copy, Clone, Debug)]
struct Matrix {
    matrix: [[i16; 1000]; 1000],
    mx: usize,
    my: usize,
}

impl Matrix {
    fn new(mx: usize, my: usize) -> Matrix {
        Matrix {
            matrix: [[0; 1000]; 1000],
            mx: mx,
            my: my,
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        self.matrix[x][y] += 1;
    }

    /*
    it's a mess but I didn't understand the quest
    */
    fn get(&self, x: i16, y: i16) -> i16 {
        let x1 = if x == -1 {
            self.mx as i16 - 1
        } else if x == self.mx as i16 {
            0
        } else {
            x as i16
        };

        let y1 = if y == -1 {
            self.my as i16 - 1
        } else if y == self.my as i16 {
            0
        } else {
            y as i16
        };

        self.matrix[x1 as usize][y1 as usize]
    }

    fn add_line(&mut self, linenum: usize, vec: Vec<char>) {
        for i in 0..self.mx {
            let n = vec.get(i).unwrap();

            let x: i16 = match n {
                '#' => 1,
                _ => 0,
            };
            self.matrix[i][linenum] = x
        }
    }

    fn get_sum(&self) -> i16 {
        let mut out: i16 = 0;
        for i in 0..self.mx {
            out += self.matrix[i].iter().map(|&x| x as i16).sum::<i16>();
            println!("{out}")
        }

        return out;
    }

    /*
    had to do check
    */
    fn check(&self) {
        for x1 in 1..self.mx - 1 {
            for y1 in 1..self.my - 1 {
                let x = x1 as i16;
                let y = y1 as i16;
                let i = self.get(x, y);
                if !(self.get(x - 1, y - 1) >= i - 1
                    && self.get(x - 1, y) >= i - 1
                    && self.get(x - 1, y + 1) >= i - 1
                    && self.get(x, y - 1) >= i - 1
                    && self.get(x, y + 1) >= i - 1
                    && self.get(x + 1, y - 1) >= i - 1
                    && self.get(x + 1, y) >= i - 1
                    && self.get(x + 1, y + 1) >= i - 1)
                {
                    println!("x:{} y:{}", x, y);
                }

                if i != 0
                    && self.get(x - 1, y - 1) >= i
                    && self.get(x - 1, y) >= i
                    && self.get(x - 1, y + 1) >= i
                    && self.get(x, y - 1) >= i
                    && self.get(x, y + 1) >= i
                    && self.get(x + 1, y - 1) >= i
                    && self.get(x + 1, y) >= i
                    && self.get(x + 1, y + 1) >= i
                {
                    println!("x:{} y:{} i:{}", x, y, i);
                }
            }
        }
    }

    fn up(&mut self, i: i16) {
        for x1 in 1..self.mx - 1 {
            for y1 in 1..self.my - 1 {
                let x = x1 as i16;
                let y = y1 as i16;

                if self.get(x, y) == i - 1
                    && self.get(x - 1, y - 1) >= i - 1
                    && self.get(x - 1, y) >= i - 1
                    && self.get(x - 1, y + 1) >= i - 1
                    && self.get(x, y - 1) >= i - 1
                    && self.get(x, y + 1) >= i - 1
                    && self.get(x + 1, y - 1) >= i - 1
                    && self.get(x + 1, y) >= i - 1
                    && self.get(x + 1, y + 1) >= i - 1
                {
                    self.set(x as usize, y as usize);
                }
            }
        }
    }
}
