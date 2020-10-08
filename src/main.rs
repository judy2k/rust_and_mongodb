mod data;

use crate::data::Recipe;
use futures::stream::StreamExt;
use mongodb::bson::{doc, from_document, Bson};
use mongodb::{options::ClientOptions, options::FindOptions, options::ResolverConfig, Client};

fn heading(s: &str) {
    println!("\n{}\n{}", s, "-".repeat(s.len()));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mdb_uri = std::env::var("MDB_URL").or(Err("MDB_URL environment variable missing"))?;

    // Parse a connection string into an options struct.
    let client_options =
        ClientOptions::parse_with_resolver_config(&mdb_uri, ResolverConfig::cloudflare()).await?;

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    let database = client.database("cocktails");

    let recipes = database.collection("recipes");

    let options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = recipes.find(None, options).await?;
    heading("All Cocktails");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                if let Some(name) = recipe.get("name").and_then(Bson::as_str) {
                    println!("Cocktail: {}", name);
                } else {
                    println!("no name found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Vodka Cocktails:
    heading("Vodka Cocktails");
    let options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = recipes
        .find(
            doc! { "ingredients": { "$elemMatch": {"name": "Vodka"} } },
            options,
        )
        .await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                if let Some(name) = recipe.get("name").and_then(Bson::as_str) {
                    println!("Cocktail: {}", name);
                } else {
                    println!("no name found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    let mut cursor = recipes.aggregate(vec![], None).await?;
    heading("Empty Aggregation Pipeline");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                if let Some(name) = recipe.get("name").and_then(Bson::as_str) {
                    println!("Cocktail: {}", name);
                } else {
                    println!("no name found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    /*
            [
        {
            "$lookup": {
                "from": "reviews",
                "localField": "_id",
                "foreignField": "recipe_id",
                "as": "reviews"
            }
        }, {
            "$addFields": {
                "review_avg": {
                    "$divide": [
                        {
                            "$round": {
                                "$multiply": [
                                    2, {
                                        "$avg": "$reviews.rating"
                                    }
                                ]
                            }
                        }, 2
                    ]
                }
            }
        }, {
            "$sort": {
                "review_avg": -1
            }
        }
    ]
            */

    // Sorting:
    let mut cursor = recipes
        .aggregate(
            vec![doc! {
                "$sort": {
                    "name": 1
                }
            }],
            None,
        )
        .await?;
    heading("Sorting");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                if let Some(name) = recipe.get("name").and_then(Bson::as_str) {
                    println!("Cocktail: {}", name);
                } else {
                    println!("no name found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Filtering:
    let query = vec![
        doc! {
            "$match": {
                "name": { "$lte": "Addison" }
            }
        },
        doc! {
            "$sort": {
                "name": 1
            }
        },
    ];

    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("Filtering");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                if let Some(name) = recipe.get("name").and_then(Bson::as_str) {
                    println!("Cocktail: {}", name);
                } else {
                    println!("no name found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Deserialization:
    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("Deserialization");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                let recipe: Recipe = from_document(recipe).unwrap();
                println!("{}", recipe);
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Join:
    let query = vec![
        doc! {
            "$match": {
                "name": { "$lte": "Addison" }
            }
        },
        doc! {
            "$sort": {
                "name": 1
            }
        },
        doc! {
            "$lookup": {
                "from": "reviews",
                "localField": "_id",
                "foreignField": "recipe_id",
                "as": "reviews",
            },
        },
    ];
    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("Lookup");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                let recipe: Recipe = from_document(recipe).unwrap();
                println!("{}", recipe);
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Aggregate:
    let query = vec![
        doc! {
            "$match": {
                "name": { "$lte": "Addison" }
            }
        },
        doc! {
            "$sort": {
                "name": 1
            }
        },
        doc! {
            "$lookup": {
                "from": "reviews",
                "localField": "_id",
                "foreignField": "recipe_id",
                "as": "reviews",
            },
        },
        doc! {
            "$addFields": {
                "rating": {
                    "$avg": "$reviews.rating",
                },
            },
        },
    ];
    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("Lookup");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                let recipe: Recipe = from_document(recipe).unwrap();
                println!("{}", recipe);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
