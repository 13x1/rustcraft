use std::fmt::Display;
use anyhow::Result;
use serde::Deserialize;

use actix_web::{error, post, Responder, web};
use actix_web::web::Json;
use serde_json::json;
use crate::com_handle::{RawCCRequestO};
use crate::state::{CallbackID, State};

pub type Data = web::Data<State>;

// helper for making actix return fancy errors instead of 500s
trait ConvertInternal<T> {
    fn to_actix(self) -> actix_web::Result<T>;
}
impl<T, E: Display> ConvertInternal<T> for Result<T, E> {
    fn to_actix(self) -> actix_web::Result<T> {
        self.map_err(|e| error::ErrorInternalServerError(format!("{}", e)))
    }
}
impl<T> ConvertInternal<T> for Option<T> {
    fn to_actix(self) -> actix_web::Result<T> {
        self.ok_or_else(|| error::ErrorNotFound("Not found"))
    }
}


#[post("/fetchTasks")]
pub async fn fetch_tasks(data: Data, body: String) -> actix_web::Result<impl Responder> {
    let computer = {
        let computer_registry = data.computer_registry.lock().await;
        computer_registry.get(&body).to_actix()?.clone()
    };

    let request = {
        let mut receiver = computer.receiver.lock().await;
        receiver.recv().await.to_actix()?
    };

    let callback_id = uuid::Uuid::new_v4().to_string();

    {
        let mut callback_registry = data.callback_registry.lock().await;
        callback_registry.insert(callback_id.clone(), request.tx);
    }

    Ok(Json(json!({
        "callback_id": callback_id,
        "lua": request.data
    })))
}


#[derive(Deserialize, Clone)]
struct CBBody {
    callback_id: CallbackID,
    data: RawCCRequestO
}

#[post("/callback")]
pub async fn callback(data: Data, body: Json<CBBody>) -> actix_web::Result<impl Responder> {
    let sender = {
        let mut callback_registry = data.callback_registry.lock().await;
        callback_registry.remove(&body.callback_id).to_actix()?
    };
    sender.send(body.clone().data).to_actix()?;

    Ok(Json(json!({
        "status": "ok"
    })))
}

#[cfg(feature = "dylib")]
pub fn a() {}