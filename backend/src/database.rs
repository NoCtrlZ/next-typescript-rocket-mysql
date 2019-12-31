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

pub fn select_temporary_user_salt(email: &str) -> (String, String) {
    let pool = my::Pool::new("mysql://shin:0523@localhost:3306/db").unwrap();
    let mut stmt = pool.prepare(r"SELECT salt, password FROM temporary_users WHERE mail = :mail").unwrap();
    let mut row = stmt.execute(params!{ "mail" => &email,}).unwrap();
    match row.next().unwrap() {
        Ok(row_salt) => {
            my::from_row(row_salt)
        },
        Err(msg) =>
            panic!("There is no record {}", msg),
    }
}
