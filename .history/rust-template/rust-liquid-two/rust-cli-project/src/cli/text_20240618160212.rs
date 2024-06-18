use crate::{
    get_content,get_reader,process_text_key_generate,process_text_sign,process_text_verify,
    CmdExector,
}

use super::{verify_file,verify_path};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD,Engine};