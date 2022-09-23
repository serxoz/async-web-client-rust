/*
 * Exemplo de HTTP GET() Asíncrono con Rust
 *
 */
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]  // discount_percentage ven como discountPercentage
struct APIResponse {
    id: i32,
    title: String,
    description: String,
    price: f32,
    discount_percentage: f32,
    rating: f32,
    stock: i32,
    brand: String,
    category: String,
    thumbnail: String,
    images: Vec<String>,
}

#[tokio::main]
async fn main() {
    let url = "https://dummyjson.com/products/1".to_string();
    let res = reqwest::get(url).await.unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {
            // si foi ben parsease o json
            match res.json::<APIResponse>().await {
                Ok(parsed) => imprimir(&parsed),
                Err(error) => println!("{:?}",error)
            }
        }
        other => {
            panic!("Explotou algo: {:?}", other);
        }
    }
}

fn imprimir(prod: &APIResponse) {
    // println!("{:?}", prod);
    println!("Producto: {}", prod.title);
}
