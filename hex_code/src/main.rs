use hex::ToHex;

fn main(){
    let a= String::from("22410516");
    let s = a.encode_hex::<String>();
    println!("{}", s);
}