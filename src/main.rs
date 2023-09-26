use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use axum::response::Html;
use minijinja::render;

mod html;
use crate::html::CONTAINERS;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
struct ContainerInfo {
    Id: String,
    Names: Vec<String>,
    Image: String,
    Status: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct FilteredContainerInfo {
    Id: String,
    Name: String,
    Image: String,
    Status: String
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/containers", get(active_containers));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn active_containers() -> Html<String>{
    let client = reqwest::Client::new();
    let url = format!("http://localhost:2375/containers/json");
    let response = client.get(url).send().await.unwrap();
    let body = response.text().await.unwrap();
    
    let container_info: Vec<ContainerInfo> = serde_json::from_str(&body).expect("NYET");
    let mut filtered_container_info: Vec<FilteredContainerInfo> = Vec::new();

    for info in &container_info {
        let container_id_long = &info.Id;
        let container_id: String = container_id_long.chars().take(12).collect();
        let unfiltered_names = &info.Names.join("");

        let container_name = unfiltered_names.replace(&['[', ']', '/', '\"'][..], "");

        let container_image = &info.Image;
        let container_status = &info.Status;
        println!("container id: {}\nname: {}\nimage: {}\nstatus: {}\n\n", container_id, container_name, container_image, container_status);
        
        let filtered_info = FilteredContainerInfo{
            Id: container_id.to_string(),
            Name: container_name.to_string(),
            Image: container_image.to_string(),
            Status: container_status.to_string()
        };

        filtered_container_info.push(filtered_info);
    }

    let a = render!(CONTAINERS, containers => filtered_container_info);

    Html(a)
}


