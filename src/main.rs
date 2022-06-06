use std::env;

struct Element {
    val: i32,
    name: Option<String>,
}

struct ByteCode {
    instructions: Vec<String>,
    stack: Vec<Element>,
}

impl ByteCode {
    fn run(mut self) {
        let i = 0;
        let command: Vec<&str> = self.instructions[i].split(' ').collect();

        match command[0].len() {
            2 => {
                match command[0] {
                    "LOAD_VAL" => {
                        let e = Element {
                            val: command[1].parse::<i32>().unwrap(),
                            name: None,
                        };
                        self.stack.push(e);
                    }
                    "WRITE_VAR" => {
                        let x = self.stack.pop().unwrap().val;

                        let v = Element {
                            val: x,
                            name: Some(command[1].to_string()),
                        };
                        self.stack.push(v);
                    }
                    "READ_VAR" => {
                        
                    }
                    &_ => {
                    }
                }
            }
            1 => {
                let a = self.stack.pop().unwrap().val;
                let b = self.stack.pop().unwrap().val;
                let mut res: Option<i32> = None;
                match command[0] {
                    "ADD" => {
                        res = Some(a + b);
                    }
                    "MULTIPLY" => {
                        res = Some(a * b);
                    }
                    "RETURN_VALUE" => {
                    }
                    //todo: sub, divide
                    &_ => {
                    }
                }
                match res {
                    Some(x) => self.stack.push(Element { val: x, name: None }),
                    None => (),
                }
            }
            _ => {}
        }
    }
}

//use PathBuf?
fn search_file(path: String) {}

fn main() {
    println!("Hello, world!");
}
