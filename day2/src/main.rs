use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    p2();
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
fn p1(){

    let mut increase = 0;
    let mut total = 0;
    let mut aim= 0;
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
                    "forward"=> {horizon+=my_depth; depth += my_depth*aim;}
                    "down" => {aim+=my_depth;}
                    "up" => {aim-=my_depth;}
                    _=> unreachable!("direction is unknown")
                };
            }
        }
    }
    println!("depth:{}, horizon:{} = {}", depth,horizon, depth*horizon);
}
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

