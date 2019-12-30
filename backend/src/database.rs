use mysql as my;
use mysql::params;

pub fn insert_temporary_user(email: &str, password: &str, salt: &str, cypher: &str) {
    let pool = my::Pool::new("mysql://shin:0523@localhost:3306/db").unwrap();
    let mut stmt = pool.prepare(r"INSERT INTO temporary_users (mail, password, salt, cypher) VALUES (:mail, :password, :salt, :cypher)").unwrap();
    stmt.execute(params!{ "mail" => email, "password" => password, "salt" => &salt, "cypher" => &cypher, }).unwrap();
    println!("{:?}", email);
    println!("{:?}", password);
    println!("{:?}", salt);
    println!("{:?}", cypher);
}
