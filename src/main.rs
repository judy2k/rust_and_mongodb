mod data;

use crate::data::Recipe;

use futures::stream::StreamExt;
use mongodb::bson::{doc, from_document};
use mongodb::{options::ClientOptions, options::FindOptions, options::ResolverConfig, Client};

fn heading(s: &str) {
    println!("\n{}\n{}", s, "-".repeat(s.len()));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mdb_uri = std::env::var("MDB_URL").or(Err("MDB_URL environment variable missing"))?;

    // Parse a connection string into an options struct.
    // This ClientOptions is configured to use the cloudflare resolver to
    // avoid a bug in the default Windows resolver.
    let client_options =
        ClientOptions::parse_with_resolver_config(&mdb_uri, ResolverConfig::cloudflare()).await?;

    // Get a handle to the cluster:
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
    // This was just sample code:
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

    // Search by name: -------------------------------------------------------
    heading("Negroni Sbagliato");
    let query = doc! { "name": "Negroni Sbagliato" };
    let mut cursor = recipes.find(query, options.clone()).await?;

    // Execute the query and loop through the results:
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    // Vodka Cocktails: ------------------------------------------------------
    heading("Vodka Cocktails");
    // Search for all recipes containing an ingredient with the name "Vodka":
    let query = doc! { "ingredients": { "$elemMatch": {"name": "Vodka"} } };

    // Execute the query and loop through the results:
    let mut cursor = recipes.find(query, options.clone()).await?;
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    // Empty Aggregation Pipeline: -------------------------------------------
    let mut cursor = recipes.aggregate(vec![], None).await?;
    heading("Empty Aggregation Pipeline");
    while let Some(result) = cursor.next().await {
        let recipe = result?;
        println!("Cocktail: {}", recipe.get_str("name")?);
    }

    // Sorting in an Aggregation Pipeline: -----------------------------------
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

    // Filtering: ------------------------------------------------------------
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

    // Deserialization with Serde: -------------------------------------------
    let mut cursor = recipes.aggregate(query.clone(), None).await?;
    heading("Deserialization");
    while let Some(result) = cursor.next().await {
        let recipe: Recipe = from_document(result?).unwrap();
        println!("{}", recipe);
    }

    // Join with reviews: ----------------------------------------------------
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

    // Find the top 10 rated cocktails: --------------------------------------
    let query = vec![
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

    /*
    // Sample code to add a review to a recipe document:
    recipes
        .update_one(
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
    */

    Ok(())
}
