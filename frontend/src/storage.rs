use serde::{Deserialize, Serialize};
use yewdux::Store;

#[derive(Debug, Default, Clone, PartialEq, Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct UserData {
    id: Option<String>,
    email_address: Option<String>,
    token: Option<String>,
}

impl UserData {
    pub fn email_address(&self) -> Option<String> {
        self.email_address.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }
}

pub fn set_email_address(dispatch: yewdux::Dispatch<UserData>, email_address: Option<String>) {
    dispatch.reduce_mut(move |store| {
        store.email_address = email_address;
    })
}

pub fn set_token(dispatch: yewdux::Dispatch<UserData>, token: Option<String>) {
    dispatch.reduce_mut(move |store| {
        store.token = token;
    })
}

pub fn set_id(dispatch: yewdux::Dispatch<UserData>, id: Option<String>) {
    dispatch.reduce_mut(move |store| {
        store.id = id;
    })
}
