// The first part of the guessing game program will ask for 
// user input, process that input, and check that the input 
// is in the expected form. To start, we’ll allow the player 
// to input a guess



// To obtain user input and then print the result as output, 
// we need to bring the io input/output library into scope.
// The io library comes from the standard library, 
// known as std
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// The fn syntax declares a new function
fn main()
{
    // println! is a macro that prints a string to the screen
    println!("Guess the number!");

    // rand::thread_rng function that gives us the particular random number generator that we’re going to use
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The scret number is {}",secret_number );

    loop 
    {
        println!("Please input your guess");

        // We use the let statement to create the variable
        // In Rust, variables are immutable by default
        let _apple = 5;  //immutable
        let mut guess = String::new(); //mutable
        // calling String::new, a function that returns a 
        // new instance of a String

        // In full, the let mut guess = String::new(); line has 
        // created a mutable variable that is currently bound 
        // to a new, empty instance of a String.




        // Now we’ll call the stdin function from the io module, 
        // which will allow us to handle user input

        // the line .read_line(&mut guess) calls the read_line method on the 
        // standard input handle to get input from the user.    
        io::stdin()
            .read_line(&mut guess) 
            .expect("failed to read line");

        let guess:u32 = guess.trim().parse().expect("please type a number");
        
        println!("You guessed: {}", guess );

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => 
            {
                println!("You win");
                break;
            }
        }
    }

    let x = 5;
    let y = 6;

    println!("x = {} and y = {}",x, y );
}


