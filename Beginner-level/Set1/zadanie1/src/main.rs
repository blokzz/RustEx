use std::io;

fn main() {
    let mut a = String::new(); 
    println!("Wprowadź pierwszą liczbę: ");
    io::stdin()
        .read_line(&mut a)
        .expect("Nie udało się odczytać linii");
    let a: i32 = a.trim().parse().expect("Proszę wprowadzić liczbę");
    let mut b = String::new(); 
    println!("Wprowadź drugą liczbę: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Nie udało się odczytać linii");
    let b: i32 = b.trim().parse().expect("Proszę wprowadzić liczbę");
    let mut op = String::new(); 
    println!("Wprowadź operator ( * , / , + , - ) : ");
    io::stdin()
        .read_line(&mut op)
        .expect("Nie udało się odczytać linii");
    let op = op.trim();
    
    if op == "*" {
        println!("{}",mnozenie(a,b));
    }
    else if op =="/"{
        if b==0 {
            println!("Nie dzielimy przez zero!");
        }
        else{
        println!("{}",dzielenie(a,b));
        }
    }
    else if op =="+"{
        println!("{}",dodawanie(a,b));
    }
    else if op =="-"{
        println!("{}",odejmowanie(a,b));
    }
    else{
        println!("Podany operator nie istnieje!");
    }
}

fn dodawanie(num1:i32, num2:i32) -> i32{
    return num1+num2;
}

fn odejmowanie(num1:i32, num2:i32) -> i32{
    return num1-num2;
}

fn mnozenie(num1:i32, num2:i32) -> i32{
    return num1*num2;
}

fn dzielenie(num1:i32, num2:i32) -> i32{
    return num1/num2;
}