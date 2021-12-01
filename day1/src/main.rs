use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    p1();
    p2();
}

fn p2(){
    let mut increase = 0;
    let mut total = 0;
    let mut last: Option<usize> = None;
    let mut last3: Vec<usize> = vec![];
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(depth) = line {
                total +=1;

                let my_depth = depth.parse::<usize>().unwrap();
                last3.push(my_depth);
                if last3.len()>3{
                    last3.remove(0);
                }
                if last3.len()==3{
                    let current_depth = last3.iter().sum();
                    let result = if let Some(last_depth) = last{
                        if current_depth> last_depth{
                            increase += 1;
                            "(increase)"
                        }else if current_depth < last_depth{
                            "(decreased)"
                        }else{
                            "(same)"
                        }
                    }else{
                        "(N/A - no previous measurement)"
                    };
                    last = Some(current_depth);
                    println!("{} {}", current_depth,result);
                }
            }
        }
    }
    println!("total:{}, increase:{}", total,increase);
}
fn p1(){
    let mut increase = 0;
    let mut total = 0;
    let mut last: Option<usize> = None;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(depth) = line {
                total +=1;

                let my_depth = depth.parse::<usize>().unwrap();
                let result = if let Some(last_depth) = last{
                    if my_depth > last_depth{
                        increase += 1;
                        "(increase)"
                    }else if my_depth < last_depth{
                        "(decreased)"
                    }else{
                        "(same)"
                    }
                }else{
                    "(N/A - no previous measurement)"
                };
                last = Some(my_depth);
                println!("{} {}", depth,result);
            }
        }
    }
    println!("total:{}, increase:{}", total,increase);
}
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

