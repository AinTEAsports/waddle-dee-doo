use reqwest;
use structopt::StructOpt;


struct URL {
    url: String
}


impl URL {
    pub fn new(url: &str) -> URL {
        return URL{ url: String::from(url) }
    }


    pub fn is_valid(&self) -> bool {
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            return true;
        }

        return false;
    }


    pub async fn is_accessible(&self) -> bool {
        let response = reqwest::get(self.url.as_str()).await.unwrap();
        return response.status().is_success();
    }


    pub fn get_url(&self) -> &String {
        return &self.url;
    }

    
    // Function to resume what happened (status code, name, ping, etc...)
    pub async fn resumed_content(&self) -> String {
        let formatted = format!("---| {} |---\nDomain name : {}\nIs accessible : {}\n\n", self.url, self.get_url(), self.is_accessible().await);

        return String::from(formatted);
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name="CliSite", about="A simple comand line tool to check if you have access to a website, and some more informations")]
struct Options {
    /// website to check if you have access
    #[structopt(short="u", long="urls")]
    urls: Vec<String>,
}



#[tokio::main]
async fn main() {
    let mut url_object: URL;
    let options = Options::from_args();

    for url in options.urls.iter() {
        url_object = URL::new(url.as_str());

        if !url_object.is_valid() {
            println!("A given URL ('{}') is invalid", url);
            return;
        }
    }


    print!("URLs to check : {:?}", options.urls);
    println!();

    for url in options.urls.iter() {
        url_object = URL::new(url.as_str());
        // println!("{} => {}", url_object.get_url(), url_object.is_accessible().await);
        println!("{}", url_object.resumed_content().await);
    }
}
