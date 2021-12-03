use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    p1();
}

fn p2(){
    let mut increase = 0;
    let mut total = 0;
    let mut depth = 0;
    let mut horizon = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line) =line{
                let command: Vec<_>= line.split(" ").collect();
                let direction = command.first().unwrap();
                let volume=command.last().unwrap();
                let my_depth = volume.parse::<usize>().unwrap();

                total +=1;
                match *direction{
                    "forward"=> {horizon+=my_depth;}
                    "down" => {depth+=my_depth;}
                    "up" => {depth-=my_depth;}
                    _=> unreachable!("direction is unknown")
                };
            }
        }
    }
    println!("depth:{}, horizon:{} = {}", depth,horizon, depth*horizon);
}
use std::convert::TryInto;
fn p1(){
    let mut count = vec![0; 50];
    let mut input_size = 0;
    let mut total =0;
    if let Ok(lines) = read_lines("./input") {

        for line in lines {
            if let Ok(line) =line{
                let bits: Vec<_>= line.chars().map(|b| { 
                    b.to_string().parse::<usize>().unwrap()}
                    ).collect();
                if input_size==0{
                    input_size = bits.len();
                    count = vec![0; input_size];
                }
                count = count.iter().zip(bits.iter()).map(|(x, y)| x + y).collect();
                total +=1;
            }
        }
        let mut max:Vec<u8> = vec![];
        let mut min:Vec<u8> = vec![];
        let avg = total/2;
        for bit in count.iter(){

            if bit > &avg{
                max.push(1);
                min.push(0);
            }else{
                max.push(0);
                min.push(1);
            }
        }
        let max_string:String= max.iter().map( |&id| id.to_string()).collect();
        let min_string:String= min.iter().map( |&id| id.to_string()).collect();
        let max_number = isize::from_str_radix(&max_string, 2).unwrap();
        let min_number = isize::from_str_radix(&min_string, 2).unwrap();
        println!("max:{}, min:{} = {}", max_number,min_number, max_number*min_number);
    }
}
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
//https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
fn vec_array<T: std::fmt::Debug, const N: usize>(v: Vec<T>) -> [T; N] {
    dbg!(&v);
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
