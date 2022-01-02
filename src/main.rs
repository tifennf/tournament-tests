use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let init = Init {
        max_player: 64,
        name: "test".to_string(),
    };

    reqwest::Client::new()
        .post("http://localhost:3024/tournament")
        .json(&init)
        .send()
        .await
        .unwrap();

    let ins_state = InsState { open: true };

    reqwest::Client::new()
        .post("http://localhost:3024/tournament/inscriptions/status")
        .json(&ins_state)
        .send()
        .await
        .unwrap();

    let mut error_list = Vec::new();

    for n in 0..64 {
        let n_str = n.to_string();

        let player = Player {
            league_name: n_str.clone(),
            discord_username: "test".to_string(),
            tag: 4,
            discord_id: n_str,
        };

        let request = reqwest::Client::new()
            .post("http://localhost:3024/tournament/inscriptions")
            .json(&player);

        let res = request.send().await.unwrap();

        if !res.status().is_success() {
            error_list.push(res.status());
        }
    }

    reqwest::get("http://localhost:3024/tournament")
        .await
        .unwrap();

    println!("error: {}", error_list.len());

    if error_list.len() > 0 {
        error_list
            .iter()
            .for_each(|err| println!("StatusCode: {}", err));
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    league_name: String,
    discord_username: String,
    tag: u16,
    discord_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InsState {
    open: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Init {
    max_player: usize,
    name: String,
}
