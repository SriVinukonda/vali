use core::fmt;

use rocket::time::{Date, OffsetDateTime};
use sqlx::types::time::PrimitiveDateTime;
use rocket::time::macros::{date,time};

use anyhow::Result;

#[macro_use] extern crate rocket;
#[derive(Debug)]
pub struct JournalLogs {
    log_id: i64,
    log_entry: Option<String>,
    tags: Option<String>,
    log_date: PrimitiveDateTime,
}

impl JournalLogs {
    pub async fn get_journal_entries() -> Result<Vec<JournalLogs>> {
        let db_url: &str = "postgres://srilc:srilc123@localhost:5432/srilc"; // Needs to be loaded from env urls before deployment
        let pool = sqlx::postgres::PgPool::connect(db_url).await?;

        let res = sqlx::query_as!(JournalLogs, "SELECT * FROM journal_logs")
            .fetch_all(&pool)
            .await?;

        // println!("Response in get_journal_entries: {res:?}");
        // let response: &str = res.get(0);

        // println!("response: {response}");
        Ok(res)
    }

    pub async fn insert_journal_entry(journal_entry: JournalLogs) -> Result<i64> {
        let db_url: &str = "postgres://srilc:srilc123@localhost:5432/srilc"; // Needs to be loaded from env urls before deployment
        let pool = sqlx::postgres::PgPool::connect(db_url).await?;
        // "INSERT INTO journal_logs (log_id,log_entry,tags,log_date) VALUES ({$journal_entry.log_id},{journal_entry.log_entry},{journal_entry.tags},{journal_entry.log_date})
        let res: (i64,) = sqlx::query_as("INSERT INTO journal_logs VALUES ($1,$2,$3,$4) returning log_id")
            .bind(journal_entry.log_id)
            .bind(journal_entry.log_entry)
            .bind(journal_entry.tags)
            .bind(journal_entry.log_date)
            .fetch_one(&pool)
            .await?;
        println!("{res:?}");
        Ok(4i64)

        
    }
}

impl fmt::Display for JournalLogs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut actual_log_entry = String::from("");
        let mut actual_tags = String::from("");
        
        match &self.log_entry {
            Some(val) => {actual_log_entry = val.to_string()}
            None => {}
        }

        match &self.tags {
            Some(val) => {actual_tags = val.to_string()}
            None => {}
        }

        write!(f,"{{
            \"log_id\": \"{}\",
            \"log_entry\": \"{}\",
            \"tags\": \"{}\",
            \"log_date\": \"{}\"
            
            }}",self.log_id,actual_log_entry,actual_tags,self.log_date)
    }
}
// impl fmt::Display for Point {
//     fn fmt(&self, f:)
// }
#[get("/")]
async fn index() -> String {
    let journal_entries = JournalLogs::get_journal_entries().await;
    let mut processed_j_ents: Vec<String> = vec![];
    let first_date = PrimitiveDateTime::MIN;

    println!("response: {journal_entries:?}");
    match journal_entries {
        Ok(j_ents) => {
            // processed_j_ent = j_ent;
            for curr_ent in j_ents {
                processed_j_ents.push(curr_ent.to_string());
            }
        }
        _ => {
            let curr_ent = JournalLogs {
                log_id: 0i64,
                log_entry: Some(String::from("new_entry")),
                tags: Some(String::from("")),
                log_date: first_date,
            };
            processed_j_ents.push(curr_ent.to_string());
        }
    }
    
    return processed_j_ents.join(",");
}

#[get("/insert")]
async fn insert_entry() -> &'static str {
    let new_entry: String = String::from("Half guard pass a) i.Grab onto belt and ensure they get an underhook. ii.Then you want to sprawl out and bring your shoulder to their chin. iii. Keep pressure on their face and and then use a tripod to get out of halfguard. iv. Rotate around to get side control. b) i. Grab their hips, then sprawl your leg out ii. Once your leg is out you can repeat the same as above. iii. Or we can step away towards the inside and get an armbar.");
    
    // PrimitiveDateTime::new(date!(2023-05-25), time!(0:00));
    let log_date_pd = PrimitiveDateTime::new(date!(2023-05-25),time!(0:00));
    let new_entry_log = JournalLogs {
        log_id: 2i64,
        log_entry: Some(new_entry),
        tags: Some(String::from("half-guard, passing")),
        log_date: log_date_pd,
    };
    let journal_entries = JournalLogs::insert_journal_entry(new_entry_log).await;
    println!("response: {journal_entries:?}");

    return "tried to insert!";
}

// #[tokio::main]
// async fn get_journal_entries() -> Result<(), Box<dyn Error>> {
//     let db_url: &str = "postgres://srilc:srilc123@localhost:5432/srilc"; // Needs to be loaded from env urls before deployment
//     let pool = sqlx::postgres::PgPool::connect(db_url).await?;
    
//     let res = sqlx::query("SELECT * FROM journal_logs")
//         .fetch_one(&pool)
//         .await?;

//     let response: &str = res.get(0);

//     println!("response: {response}");
//     Ok(())
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,insert_entry])

}