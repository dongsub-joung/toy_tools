
fn main() {
    let set_date= "2021 03 02";
    let mut list= set_date.split_whitespace();
    
    let yyyy= list.next().unwrap();
    let mm= list.next().unwrap();
    let dd= list.next().unwrap();

    let (t_yyyy, t_mm, t_dd)= getToDay();

    let yyyy: i32= yyyy.parse().unwrap();
    let mm: i32= mm.parse().unwrap();
    let dd: i32= dd.parse().unwrap();

    let t_yyyy: i32= t_yyyy.parse().unwrap();
    let t_mm: i32= t_mm.parse().unwrap();
    let t_dd: i32= t_dd.parse().unwrap();

    let r_yy= (yyyy-t_yyyy).abs();
    let r_mm= (mm-t_mm).abs();
    let r_dd= (dd-t_dd).abs();

    let result= format!("Remaining {} : {} : {}", r_yy, r_mm, r_dd);
    
    println!("{}", result);
}

fn getToDay() -> (String, String, String){
    let mut yyyy= String::new();
    let mut mm= String::new();
    let mut dd= String::new();
    
    (yyyy, mm, dd)
}
