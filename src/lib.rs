use std::error::Error as stdError;


pub async fn dump_database(url: &str) -> Result<String, Box<dyn stdError>> {
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    let row: Vec<(String, String, String, String, String,)> = sqlx::query_as("SELECT * FROM kunde")
        .fetch_all(&pool).await?;

    let mut arr = "".to_string();

    for s in row {
        let single_row = format!("{}|{}|{}|{}|{}\n", s.0, s.1, s.2, s.3, s.4);
        arr = format!("{arr}{single_row}");
    }

    return Ok(arr)
}




pub async fn del_customer_by_status(status: &str, url: &str) -> Result<(), Box<dyn stdError>> {
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    let query = "DELETE FROM kunde WHERE Status = $1";

    sqlx::query(query)
        .bind(&status.to_string())
        .execute(&pool).await?;
    Ok(())
}



pub async fn del_customer(s: &str, url: &str) -> Result<(), Box<dyn stdError>> {
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    let data = json::parse(&s).unwrap();

    //{id, vorname, nachname, mail, subscription, top, middl, bottom, status"}
    let query = "DELETE FROM kunde WHERE Status = $1";

    sqlx::query(query)
        .bind(&status.to_string())
        .execute(&pool).await?;
    Ok(())
}



pub async fn add_customer(c_string: String, url: &str) -> Result<(), Box<dyn stdError>> {
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    let parts = c_string.split("|");
    let data: Vec<&str> = parts.collect();

    let query = "INSERT INTO kunde (Kundennummer, Name, Email, Nachricht, Status) VALUES ($1, $2, $3, $4, $5)";
    sqlx::query(query)
        .bind("0".to_string())
        .bind(&data[0].to_string())
        .bind(&data[1].to_string())
        .bind(&data[2].to_string())
        .bind("nix".to_string())
        .execute(&pool).await?;
    Ok(())
}

