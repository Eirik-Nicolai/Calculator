use std::io::{stdin,stdout,Write};


struct Calculation{
    left: i32,
    right: i32,
    operator: char
}

fn get_integer(inp:&str) -> Option<i32>
{
    let i = inp.parse::<i32>();
    
    match i {
        Ok(x) => {
            println!("Ok {}", x);
            Some(x)
        }
        Err(e) => {
            println!("Err {}", e);
            None
        }
    }
}

fn string_to_calculation(inp:&str) -> Calculation
{
    let operators = "+-/*%"; 
    let mut c = Calculation {
        left: 0,
        right: 0,
        operator: 'a'
    };

    let split = inp.split_whitespace();

    let vec: Vec<&str> = split.collect();

    let mut err:bool = false;
 
    match get_integer(vec[0]) {
        Some(x) => {
            c.left = x
        }
        None => {
            println!("Something Went Wrong with Parsing: {}", vec[0]);
            err = true;
        }
    }
    match get_integer(vec[2]) {
        Some(x) => {
            c.right = x
        }
        None => {
            println!("Something Went Wrong with Parsing: {}", vec[2]);
            err = true;
        }
    }

    if operators.contains(vec[1]) {
        c.operator = vec[1].chars().next().unwrap();
    }
    else
    {
        println!("Operators were fucked: {}", vec[1]);
        err = true;
    }
    

    if err {
        println!("ERR");
        c = Calculation{left:0,right:0,operator:'e'};
    }

    return c;
}


fn main() {
    loop
    {
        let mut s=String::new();
        print!("Please enter your calulation:");
        let _=stdout().flush();
        
        stdin().read_line(&mut s).expect("Incorrect");
    
        if let Some('\n')=s.chars().next_back()
        {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back()
        {
            s.pop();
        }

        println!("s: {}", s);

        if s=="stop" {
            break;
        }
    
        let c = string_to_calculation(&s);
    
        if c.operator != 'e'
        {
            
            println!("Calculating ...");
    
            let result:f32 = match c.operator {
                '+' => c.left as f32 + c.right as f32,
                '-' => c.left as f32 - c.right as f32,
                '/' => c.left as f32 / c.right as f32,
                '*' => c.left as f32 * c.right as f32,
                '%' => c.left as f32 % c.right as f32,
                _ => panic!("ERR")
            };
            
            println!("You typed {}", s);
            println!("The result was {}", result);
        }
        else
        {
            panic!("Something went wrong with the operator ({}), aborting...", c.operator);
        }
    }
}