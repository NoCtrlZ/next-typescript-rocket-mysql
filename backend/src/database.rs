use mysql as my;
use mysql::params;

pub fn insert_temporary_user(email: &str, password: &str) {
    let pool = my::Pool::new("mysql://shin:0523@localhost:3306/db").unwrap();
    let mut stmt = pool.prepare(r"INSERT INTO temporary_users (mail, password) VALUES (:mail, :password)").unwrap();
    stmt.execute(params!{ "mail" => email, "password" => password, }).unwrap();
    println!("{:?}", email);
    println!("{:?}", password);
}
