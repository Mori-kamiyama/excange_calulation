use std::io;

fn main() {
    println!("--This is created by Yuta in 2024 We cannot guarantee that this reflects current exchange rates.--");
    let usd :f64 =160.49;
    let eur :f64 = 173.88;
    let jpy :f64 = 1.00;

    loop {
        println!("\n\nPlease choice In currency");
        println!("USD: America's currency\nEUR: EU's currency\nJPY: Japan's currency");

        let mut currency_in = String::new();

        io::stdin()
            .read_line(&mut currency_in)
            .expect("Failed to read line");

        let currency_in = currency_in.trim();

        let in_cur = match currency_in {
            "JPY" => jpy,
            "USD" => usd,
            "EUR" => eur,
            _ => {
                println!("Invalid input currency. Please try again.");
                continue;
            }
        };

        println!("\n\nPlease choice Out currency");
        println!("USD: America's currency\nEUR: EU's currency\nJPY: Japan's currency");

        let mut currency_out = String::new();

        io::stdin()
            .read_line(&mut currency_out)
            .expect("Failed to read line");

        let currency_out = currency_out.trim();

        let out_cur = match currency_out {
            "JPY" => jpy,
            "USD" => usd,
            "EUR" => eur,
            _ => {
                println!("Invalid output currency. Please try again.");
                continue;
            }
        };


        println!("How much money?");
        let mut num_in = String::new();
        io::stdin()
            .read_line(&mut num_in)
            .expect("Failed to read line");

        let num_in: f64 = match num_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid amount. Please enter a valid number.");
                continue;
            }
        };

        let trans = (in_cur * num_in) / out_cur;

        println!("{}=>{} :{}", currency_in, currency_out, trans);

        println!("please type OK");
        loop {
            let mut check = String::new();

            io::stdin()
                .read_line(&mut check)
                .expect("Failed to read line");

            let check = check.trim();

            if check == "OK" {
                break;
            }
        }
    }
}
