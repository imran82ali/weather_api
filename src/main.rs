extern crate chrono;
use chrono::prelude::*;

use async_std::task;
use surf;

// fetch data from a url and return the results as a string.
// if an error occurs, return the error.
async fn fetch(url: &str) -> Result<String, surf::Exception> {
    // let mut res = surf::post(url).await?;
    
    // //let mut res = surf::get(url).await?;
    
    // println!("{:?}", res);


    surf::get(url).recv_string().await
}

// execute the fetch function and print the results
async fn execute() {
    let key="7cbb04e546f1a641693d522fdee48dce".to_string();
    let city="Karachi".to_string();
    let mut url = "http://api.openweathermap.org/data/2.5/weather?q=".to_string();
    url.push_str(&city);
    url.push_str("&units=metric&APPID=");
    url.push_str(&key);

    match fetch(&url).await {
        Ok(s) => {//println!("Fetched results: {:#?}", s);
        println!("\n We are Feteching Live weather updates from openweathermap.org \n ");
        let data: Vec<&str> = s.split("{").collect();
        //println!("{:#?}",data);
        match data.get(6) {
            Some(p)=> {
            let temp = &p[14..24];
            let datetime = match temp.parse::<i64>(){
                Ok(dt) => dt+18000,    
                Err(_) => 0
            }; 
            //println!("{}",datetime+18000);
            let naive = NaiveDateTime::from_timestamp(datetime, 0);
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
            println!("Date Time : {}", newdate);
        },//end of some
            None => println!("Data Not found")
        };
        match data.get(7) {
             Some(p)=> {println!("City Name : {}", &p[114..120]);},
             None => println!("Data Not found")
         };

          match data.get(4) {
             Some(p)=> {println!("Temp is {} C", &p[7..9]);},
             None => println!("Data Not found")
         };
            
             },
        Err(e) => println!("Got an error: {:?}", e),
    };
    
}
//karachi 1174872

fn main() {
    task::block_on(execute());
    // ^ start the future and wait for it to finish
}