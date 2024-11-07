/*
    Author: Narongchai Cherdchoo
    Date: 2024/11/07
*/
use reqwest;
use tokio;

#[warn(unused_variables)]
async fn tokio_get_target(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    //println!("Response from {}: {}", url, content);
    Ok(())
}

#[warn(unused_variables)]
fn thread_get_target(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;
    //println!("Response: {}", content);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[tokio::test]
    async fn test_get_target() {

        // Start the timer
        let start = Instant::now();

        /* Fetch the targets */
        let handle1 = tokio::spawn(async {
            if let Err(e) = tokio_get_target("https://www.180.no/search/all?w=uit").await {
                eprintln!("Error fetching target: {}", e);
            }
        });
    
        let handle2 = tokio::spawn(async {
            if let Err(e) = tokio_get_target("https://www.180.no/search/all?w=uis").await {
                eprintln!("Error fetching target: {}", e);
            }
        });
    
        // Wait for both tasks to complete
        let _ = tokio::try_join!(handle1, handle2);
        
        // Calculate the time taken
        let duration = start.elapsed();
        println!("Tokio took: {:?}", duration);
    }

    #[test]
    fn test_thread_get_target() {

        // Start the timer
        let start = Instant::now();

        /* Fetch the targets */
        let handle1 = thread::spawn(|| {
            if let Err(e) = thread_get_target("https://www.180.no/search/all?w=uit") {
                eprintln!("Error fetching target: {}", e);
            }
        });
    
        let handle2 = thread::spawn(|| {
            if let Err(e) = thread_get_target("https://www.180.no/search/all?w=uis") {
                eprintln!("Error fetching target: {}", e);
            }
        });

        let handle3 = thread::spawn(|| {
            if let Err(e) = thread_get_target("https://www.180.no/search/all?w=uio") {
                eprintln!("Error fetching target: {}", e);
            }
        });
    
        // Wait for all threads to finish
        let _ = handle1.join();
        let _ = handle2.join();
        let _ = handle3.join();
        
        // Calculate the time taken
        let duration2 = start.elapsed();
        println!("Thread took: {:?}", duration2);
    }
}
