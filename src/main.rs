mod data;

use crate::data::Recipe;

use chrono::Utc;
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
    let mut cursor = recipes.find(None, options.clone()).await?;
    heading("All Cocktails");
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    /*
    recipes.insert_one(doc! {
        "name": "Dodgy Cocktail",
        "ingredients": [{
            "name": "Water",
            "quantity": { "unit": "ml", "amount": 30 }
        }],
        "instructions": [
            "Pour yourself some water from the tap."
        ]
    }, None).await?;
    */

    // Search by name:
    heading("Negroni Sbagliato");
    let query = doc! { "name": "Negroni Sbagliato" };
    let mut cursor = recipes.find(query, options.clone()).await?;
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    // Vodka Cocktails:
    heading("Vodka Cocktails");
    let query = doc! { "ingredients": { "$elemMatch": {"name": "Vodka"} } };
    let mut cursor = recipes.find(query, options.clone()).await?;
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    let mut cursor = recipes.aggregate(vec![], None).await?;
    heading("Empty Aggregation Pipeline");
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
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
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
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
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    // Deserialization:
    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("Deserialization");
    while let Some(result) = cursor.next().await {
        let recipe: Recipe = from_document(result?).unwrap();
        println!("{}", recipe);
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

    let query = vec![
        doc! {
        "$match": {
            "ingredients": {
                "$elemMatch": {
                    "name": "Vodka"}}}},
        doc! {
        "$lookup": {
            "from": "reviews",
            "localField": "_id",
            "foreignField": "recipe_id",
            "as": "reviews"}},
    ];

    // Aggregate:
    let query = vec![
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
        doc! {
            "$sort": {
                "rating": -1
            }
        },
        doc! {
            "$limit": 10
        },
    ];
    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("10 Highest Reviewed");
    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                let recipe: Recipe = from_document(recipe)?;
                println!("{}", recipe);
            }
            Err(e) => return Err(e.into()),
        }
    }

    recipes.update_one(
        doc! {"name": "Negroni Sbagliato"},
        doc! {
            "$push": {
                "reviews": {
                    "rating": 4,
                    "when": Utc::now(),
                }
            },
        },
        None,
    )
    .await?;

    Ok(())
}
