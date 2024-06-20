use crate::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD},
    Engine as _,
}