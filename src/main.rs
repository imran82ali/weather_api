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
    // my api key 7cbb04e546f1a641693d522fdee48dce
    match fetch("http://api.openweathermap.org/data/2.5/weather?q=Karachi&units=metric&APPID=7cbb04e546f1a641693d522fdee48dce").await {
        Ok(s) => {//println!("Fetched results: {:#?}", s);
            
        let data: Vec<&str> = s.split("{").collect();
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