const DB_USER_NAME: &'static str= "root";
const DB_PASSWORD: &'static str = "miyukishiba";
const DB_IP: &'static str       = "localhost";
const PORT: &'static str        = 3306_u16;
const DB_NAME: &'static str     = "ENGEL";

fn main() {
    let db_url= format!("mysql://{}:{}@{}:{}/{}",
        DB_USER_NAME, DB_PASSWORD, DB_IP, PORT, DB_NAME);
    let pool= mysql::Pool::new(db_url).expect("faild");
    let query= "select * from engel";
    let results= pool.prep_exec(query, ()).expect("faild query");

    for row in results{
        let (income, food)= mysql::from_row(row.unwrap());
        println!("engel rate: {}", food/income);
    }
}
