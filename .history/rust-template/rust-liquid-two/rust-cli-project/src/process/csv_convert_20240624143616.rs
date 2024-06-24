use anyhow::Result;
use csv::Reader;
use serde::{Deserialize,Serialize};
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="PascalCase")]

struct Player {
    name: String,
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    nationality:String,
    #[serde(rename="Kit Number")]
    kit:u8,
}



struct fn process_csv(input:&str,output:String,format:OutputFormat) ->Result<()>{

    let mut reader:Reader<File> = Reader::from_path(input)?;
    let mut ret:Vec<Value> = Vec::with_capacity(128);
    let headers:StrngRecord = reader.headers()?.clone();

    for result:Result<StringRecord,Error> in reader.records(){

        let record : StringRecord = result?;

        let json_value:Value = headers.iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content:String = match format {
         OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?, 
    };

    fs::write!(output,content )?;
    Ok(())
    
}