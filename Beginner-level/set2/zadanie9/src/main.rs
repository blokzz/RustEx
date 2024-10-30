fn divide(a:i32,b:i32) -> Result<i32 , String>{
    if b == 0 {
        Err(String::from("dzielenie przez 0 !!!"))
    }
    else{
        Ok(a/b)
    }
} 
fn main() {
    let result = divide(10, 2);
    println!("{:?}", result);
}

