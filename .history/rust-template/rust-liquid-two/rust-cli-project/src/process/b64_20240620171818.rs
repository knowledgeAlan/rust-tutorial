use crate::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD},
    Engine as _,
};

use std::io::Read;

pub fn process_endcode(reader:&mut dyn Read,format:Base64Format) -> Result<String>{

    let mut buf:Vec<u8> = Vec::new();

    reader.read_to_end(&mut buf)?;

    let encoded:String = match format {
        Base64Format::Standard => STANDARD.encode(&buf);
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    Ok(encoded);
}

pub fn process_decode(reader:&mut dyn Read,format:Base64Format) -> Result<String>{

    let mut buf:String = String::new();

    reader.read_to_string(&mut buf)?;

    let buf:&str = buf.trim();

    let decoded:Vec<u8> = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };

    Ok(String::from_utf8(decoded)?);

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode()-> Result<()> {

        let input:&str = "Cargo.toml";


        let mut reader:Box<dyn Read> = get_reader(input);
        let format:Base64Format = Base64Format::Standard;
        assert!(process_encode(&mut reader,format).is_ok());
        Ok(());
    }
}