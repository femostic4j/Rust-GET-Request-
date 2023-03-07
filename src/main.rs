/* use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    //let res = reqwest::get("http://pbin.org/get").await?;
    let res = reqwest::get("https://xdc.blocksscan.io/api/blocks").await?;    
    println!("Status:{}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
} */


use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://xdc.blocksscan.io/api/accounts?page=2&limit=2").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}