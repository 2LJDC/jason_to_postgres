use std::error::Error as stdError;


async fn dump_database(url: &str) -> String {
    let pool = match sqlx::postgres::PgPool::connect(&url).await {
        Ok(p) => p,
        Err(_) => return "no database".to_string(),
        //Err(_) => panic!("lel1"),
    };

    let row: Vec<(String, String, String, String, String,)> = sqlx::query_as("SELECT * FROM kunde")
        .fetch_all(&pool).await.expect("nono square");

    let mut arr = "".to_string();

    for s in row {
        let single_row = format!("{}|{}|{}|{}|{}\n", s.0, s.1, s.2, s.3, s.4);
        arr = format!("{arr}{single_row}");
    }

    return arr
}




async fn del_customer(c_string: String, url: String) -> Result<(), Box<dyn stdError>> {
    let pool = match sqlx::postgres::PgPool::connect(&url).await {
        Ok(p) => p,
        Err(e) => return Err(Box::new(e)),
    };

    let parts = c_string.split("|");
    let data: Vec<&str> = parts.collect();

    //let query = "INSERT INTO kunde (Nachricht) VALUES ($1)";
    let query = "DELETE FROM kunde WHERE (Nachricht) VALUES ($1)";

    match sqlx::query(query)
        .bind(&data[2].to_string())
        .execute(&pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }

}




async fn add_customer(c_string: String, url: String) -> Result<(), Box<dyn stdError>> {
    let pool = match sqlx::postgres::PgPool::connect(&url).await {
        Ok(p) => p,
        Err(e) => return Err(Box::new(e)),
    };

    let parts = c_string.split("|");
    let data: Vec<&str> = parts.collect();

    let query = "INSERT INTO kunde (Kundennummer, Name, Email, Nachricht, Status) VALUES ($1, $2, $3, $4, $5)";
    match sqlx::query(query)
        .bind("0".to_string())
        .bind(&data[0].to_string())
        .bind(&data[1].to_string())
        .bind(&data[2].to_string())
        .bind("nix".to_string())
        .execute(&pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }

}

