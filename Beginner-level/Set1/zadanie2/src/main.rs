use std::io;

fn main() {
    let mut tem = String::new(); 
    println!("Podaj temperaturę : ");
    io::stdin()
        .read_line(&mut tem)
        .expect("Nie udało się odczytać linii");
    let tem: i32 = tem.trim().parse().expect("Proszę wprowadzić liczbę");
    let mut stopnie = String::new(); 
    println!("Konwertować na (F)ahrenheita czy (C)elsjusza? ");
    io::stdin()
        .read_line(&mut stopnie)
        .expect("Nie udało się odczytać linii");
    let stopnie = stopnie.trim().to_lowercase();
    
   println!( "{}",konwersja(tem , stopnie));

}

fn konwersja(num:i32, b:String) -> String{
    if b == "f"{
       let num = num *9/5 + 32;
       return format!("{} F" , num);
    }
    else if b =="c"{
        let num = (num-32) * 5/9;
        return format!("{} C" , num);
    }
    else{
        return format!("{}","błąd");
    }
    
}
