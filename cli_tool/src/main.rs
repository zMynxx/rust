extern crate clap;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};
use clap::{App, Arg};
use std::process;

async fn ecrList() {
    let matches = App::new("Simple CLI tool")
        .version("1.0")
        .author("Lior Dux")
        .about("Lists images in an ECR repository")
        .arg(
            Arg::with_name("repository")
                .short("r")
                .long("repository")
                .help("Sets the ECR repository name")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("profile")
                .short("p")
                .long("profile")
                .takes_value(true)
                .required(false)
                .help("aws profile to use"),
        )
        .arg(
            Arg::with_name("region")
                .short("r")
                .long("region")
                .takes_value(true)
                .required(false)
                .help("aws region"),
        )
        .get_matches();

    // Parse arguments or set defaults
    let repository = matches.value_of("repository").unwrap_or("rust101");
    let region = matches.value_of("region").unwrap_or("us-east-1");
    let profile = matches.value_of("profile").unwrap_or("default");

    // Configure AWS credentials provider based on the provided profile and region
    let credentials_provider = rusoto_core::ProfileProvider::new().with_profile(profile);

    println!("Repository: {}", repository);
    println!("Profile: {}", profile);
    println!("Region: {}", region);

    // Create an ECR client
    let client = EcrClient::new_with(
        HttpClient::new().unwrap(),
        credentials_provider,
        region.clone(),
    );

    // Create a request to describe images in the ECR repository
    let request = DescribeImagesRequest {
        repository_name: repository.to_string(),
        ..Default::default()
    };

    // Send the request and handle errors
    match client.describe_images(request).await {
        Ok(response) => {
            for image in response.image_details.unwrap_or_default() {
                println!("Image: {:?}", image);
            }
        }
        Err(error) => {
            eprintln!("Error describing images: {:?}", error);
            process::exit(1);
        }
    }
}

/// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp = client.list_tables().send().await?;

    println!("Tables:");

    let names = resp.table_names().unwrap_or_default();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}
