struct Post{
    title: String,
    body: String,
}

fn main(){
    let origin= "https://";
    let google= "google.com";
    let full_addr= format!("{}{}", origin,google);

    let addr= "https://google.com";
    let method= "POST";
    
    let conn= connection(full_addr, method, addr);

    // type inforcing 
    let title= "title".to_string();
    let body= "body".to_string();
    if conn.contains("Ok"){
        let post= Post{
            title, body
        };
    }
}

fn connection(full_addr: String, method: &'static str, addr: &'static str) -> String{ 
    let https= &addr[0..4];
    if https == "https"{
        let status= match method {
            "POST" => "Ok",
            _ => "Weak"
        };
        if full_addr == addr{
            return format!("{} connect", status);
        } else {
            return format!("{} connect", status);
        }
    } else {
        let status= "Weak";
        return format!("{} connect", status);
    }
}