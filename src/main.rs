#[derive(Debug)]
struct Shape {
    side1: i32,
    side2: i32,
}

fn main() {
    println!("Rectangle Shape Calculator! Enter the side lengths.");
    loop {
        //Starts a loop so that if an 'Err(_)' occurs, it will continue the loop and rerun.
        let mut line = String::new();
        println!("Enter Number 1:");
        let _unused = std::io::stdin().read_line(&mut line).unwrap();
        let line: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut line2 = String::new(); //Line & line2 are mutable user generated strings that we put into shape1.
        println!("Enter Number 2:");
        let _unused = std::io::stdin().read_line(&mut line2).unwrap();
        let line2: i32 = match line2.trim().parse() {
            Ok(num) => num, //If the result is a num aka if its ok then it'll pass.
            Err(_) => {
                continue;
            } // '_' Catches all errors and continues not matter what.
        };

        let shape1 = Shape {
            side1: line, //Taking the struct and putting the gathered values from user inputs into it.
            side2: line2,
        };

        println!("Area of shape: {}", shape1.side1 * shape1.side2);
        let mut always_false = false;
        loop {
            let mut answer = String::new();
            println!("Do you want to calculate another rectangle or stop? (Y/N)");

            let _unused = std::io::stdin().read_line(&mut answer).unwrap();
            let answer: &str = answer.trim();
            if answer == "Y" {
                always_false = true;
                break;
            } else if answer == "N" {
                break;
            } else {
                println!("Please enter a valid answer. (Y/N)");
                continue;
            }
        }
        if always_false {
            continue;
        } else {
            break;
        }
    }
}
