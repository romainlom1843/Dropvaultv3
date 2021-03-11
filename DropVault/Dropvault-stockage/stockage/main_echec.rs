use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = HashMap<i32, String>;
fscnfeknjefsfefes
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Password {
    id: i32,
    passwd: String,
}

//Création et implementation map
#[derive(Clone)]
struct Map {
  name_list: Arc<RwLock<Items>>,
  mail_list: Arc<RwLock<Items>>,
  passwd_list: Arc<RwLock<Items>>,
}

impl Map {
    fn new() -> Self {
        Map {
            name_list: Arc::new(RwLock::new(Items::new())),
            mail_list: Arc::new(RwLock::new(Items::new())),
            passwd_list: Arc::new(RwLock::new(Items::new())),
        }
    }
}

async fn update_name_list(
    name: Name,
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        map.name_list.write().insert(name.id, name.name);

        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

async fn update_mail_list(
    mail: Mail,
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        map.mail_list.write().insert(mail.id, mail.name);

        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

async fn update_passwd_list(
    passwd: Password,
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        map.passwd_list.write().insert(passwd.id, passwd.passwd);

        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

async fn delete_name_list_item(
    id: Id,
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        map.name_list.write().remove(&id.name);
    
        Ok(warp::reply::with_status(
            "Removed item from grocery list",
            http::StatusCode::OK,
        ))
}

async fn get_name_list(
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = HashMap::new();
        let r = map.name_list.read();
    
        for (key,value) in r.iter() {
            result.insert(key, value);
        }

        Ok(warp::reply::json(
            &result
        ))
}

async fn get_mail_list(
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = HashMap::new();
        let r = map.mail_list.read();
    
        for (key,value) in r.iter() {
            result.insert(key, value);
        }

        Ok(warp::reply::json(
            &result
        ))
}

async fn get_passwd_list(
    map: Map
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = HashMap::new();
        let r = map.passwd_list.read();
    
        for (key,value) in r.iter() {
            result.insert(key, value);
        }

        Ok(warp::reply::json(
            &result
        ))
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


fn post1_json() -> impl Filter<Extract = (Name), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post2_json() -> impl Filter<Extract = (Mail), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post3_json() -> impl Filter<Extract = (Password), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let map = Map::new();
    let map_filter = warp::any().map(move || map.clone());

    let add_items1 = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json1())
        .and(map_filter.clone())
        .and_then(update_name_list);

    let add_items2 = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json2())
        .and(map_filter.clone())
        .and_then(update_mail_list);

    let add_items3 = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json3())
        .and(map_filter.clone())
        .and_then(update_passwd_list);

    let get_items_name = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(map_filter.clone())
        .and_then(get_name_list);

    let get_items_mail = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(map_filter.clone())
        .and_then(get_mail_list);
    
    let get_items_passwd = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(map_filter.clone())
        .and_then(get_passwd_list);

    /*let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(map_filter.clone())
        .and_then(delete_map_list_item);
    
    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(map_filter.clone())
        .and_then(update_map_list);*/

    
    let routes = add_items1 .and(add_items2) .and(add_items3) .or(get_items_name) .or(get_items_mail).or(get_items_passwd) .or(delete_item).or(update_item) ;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
