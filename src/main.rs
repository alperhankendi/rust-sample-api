use warp::{http, Filter, delete};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
    basket_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            basket_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn update_basket_list(
    item: Item,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.basket_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the basket list",
        http::StatusCode::CREATED,
    ))
}

async fn delete_basket_list_item(
    id: Id,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.basket_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed item from basket list",
        http::StatusCode::OK,
    ))
}

async fn get_basket_list(
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = store.basket_list.read();

    for (key,value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(
        &result
    ))
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {

    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("basket"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_basket_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("basket"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_basket_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("basket"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_basket_list_item);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("basket"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_basket_list);


    let routes = add_items.
                                        or(get_items).
                                        or(delete_item).
                                        or(update_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}