use std::io;

fn main() {
    println!("\nTRANSFORMAR FAHRENHEIT PARA CELSIUS\n");

    println!("
    1 - TRANSFORMAR FAHRENHEIT PARA CELSIUS
    0 - TRANFORMAR CELSIUS PARA FAHRENHEIT
    ");

    loop {
    println!("Escolha uma das opções a cima: ");
    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Choice a valid input");

    let choice: i32 = choice.trim().parse()
        .expect("error");
    
        println!("Qual o valor da temperatura?");

        let mut temp = String::new();
        match io::stdin().read_line(&mut temp) {
            Ok(_) => {},
            Err(_) => {
                println!("Failed to read temperature!");
                return;
            }
        }

        // trim() elimita espaços na strig (\n)
        // .parse() transformar string em numero de fato
        let temp: f64 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_e) => {
                panic!("that wasn't valid input! Temperatures can only be integers!");
            }
        };

        if choice == 1 {
            let res = ftoc(temp);
            println!(" {}° Celsius", res);
            break;
        } if choice == 0 {
            let res = ctof(temp);
            println!("{}° Fahrenheit", res);
            break;
        } else {
            println!("Invalid input:
            1 - TRANSFORMAR FAHRENHEIT PARA CELSIUS
            0 - TRANFORMAR CELSIUS PARA FAHRENHEIT
            ");
            continue;

        }
    }

}

fn ftoc(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn ctof(c: f64) -> f64 {
    (c * (9.0/5.0)) + 32.0
}
