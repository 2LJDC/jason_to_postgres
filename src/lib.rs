use std::error::Error as stdError;
use json::object;

pub async fn dump_database(url: &str) -> Result<String, Box<dyn stdError>> {
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    let row: Vec<(String, String, String, String, String, String, String, String, String)> = sqlx::query_as("SELECT * FROM kunde")
        .fetch_all(&pool).await?;

    let mut arr = r#"{"arr":["#.to_string();

    for s in row {
        //let single_row = format!("{}|{}|{}|{}|{}\n", s.0, s.1, s.2, s.3, s.4);
        //let single_row = format!(r#"{\n id: "{}"\n vorname: "{}",\n nachneme: "{}",\n mail: "{},\n subscription:"{}",\n top: "{}",\n middle: "{}",\n bottom: "{}",\n status: "{}"\n } "#, s.0, s.1, s.2, s.3, s.4, s.5, s.6, s.7, s.8, s.9);
        let single_row = object!{
            id: s.0,
            vorname: s.1,
            nachname: s.2,
            mail: s.3,
            subscription: s.4,
            top: s.5,
            middle: s.6,
            bottom: s.7,
            status: s.8,
        };
        arr.remove(arr.len() - 1);
        arr = format!("{arr}{},", single_row);
    }
    
    arr = format!("{}{}", arr, "]}");
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

    //{id, vorname, nachname, mail, subscription, top, middle, bottom, status"}
    let query = "DELETE FROM kunde WHERE id = $1 AND vorname = $2 AND nachname = $3 AND mail = $4 AND subscription = $5 AND top = $6 AND middle = $7 AND bottom = $8 AND status = $9";

    sqlx::query(query)
        .bind(&data["id"].to_string())
        .bind(&data["vorname"].to_string())
        .bind(&data["nachname"].to_string())
        .bind(&data["mail"].to_string())
        .bind(&data["subscription"].to_string())
        .bind(&data["top"].to_string())
        .bind(&data["middle"].to_string())
        .bind(&data["bottom"].to_string())
        .bind(&data["status"].to_string())
        .execute(&pool).await?;
    Ok(())
}



pub async fn add_customer(s: String, url: &str) -> Result<(), Box<dyn stdError>> {
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    //let parts = c_string.split("|");
    //let data: Vec<&str> = parts.collect();

    let data = json::parse(&s).unwrap();

    //let query = "INSERT INTO kunde (Kundennummer, Name, Email, Nachricht, Status) VALUES ($1, $2, $3, $4, $5)";
    let query = "INSERT INTO kunde (id, vorname, nachname, mail, subscription, top, middle, bottom, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

    sqlx::query(query)
        .bind(&data["id"].to_string())
        .bind(&data["vorname"].to_string())
        .bind(&data["nachname"].to_string())
        .bind(&data["mail"].to_string())
        .bind(&data["subscription"].to_string())
        .bind(&data["top"].to_string())
        .bind(&data["middle"].to_string())
        .bind(&data["bottom"].to_string())
        .bind(&data["status"].to_string())
        .execute(&pool).await?;
    Ok(())
}
        /*.bind("0".to_string())
        .bind(&data[0].to_string())
        .bind(&data[1].to_string())
        .bind(&data[2].to_string())
        .bind("nix".to_string())
        .execute(&pool).await?;
    Ok(())
    */
//}

