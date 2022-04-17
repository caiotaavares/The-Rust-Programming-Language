use std::io;

fn main() {
    println!(   "Qual termo da sequência de fibonacci
                você deseja calcular?");
    
    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Please, choice a valid input");

    let choice: i32 = match choice.trim().parse() {
        Ok(temp) => temp,
        Err(_e) => {
            panic!("that wasn't valid input! can only be integers!");
        }
    };

    println!("O resultado é: {}", fib(choice));
}

fn fib(n: i32) -> i32 {

    if n == 1 {return 1}
    else 
        if n == 2 {return 1}
        else {return fib(n - 1) + fib(n - 2)};


    // let mut fib1 = 1;
    // let mut fib2 = 1;

    // for _n in 3..n {
    //     let soma = fib1 + fib2;
    //     fib1 = fib2;
    //     fib2 = soma;
    // }
    // return fib2;
}
