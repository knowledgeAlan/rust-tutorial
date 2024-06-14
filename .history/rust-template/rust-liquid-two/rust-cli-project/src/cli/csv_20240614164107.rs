use crate::CmDExcetor;

use super::verify_file;
use clap::Parser;
use std::{fmt,str::FromStr};

#[derive(Debug,Clone,Copy)]


pub enum OutputFormat {

    Json,
    Yaml,
}

#[derive(Debug,Parser)]

pub struct CsvOps {

    #[arg(short,long,value_parser=verify_file)])]
    pub input:String,

    #[arg(short,long)]
    pub output:Option<String>,

    #[arg(long,value_parser=parse_format,default_value="json")]
    pub format:OutputFormat,

    #[arg(short,long,default_value_t=",")]
    pub delimiter:char,

    #[arg(long,default_value_t=true)]
    pub header:bool,

}


impl CmdExector for CsvOps {

    async fn execute(self) -> anyhow::Result<()>{

        let output:String = if let Some(output) = self.output{
            output
        }else{
            format!("output.{}",self.format);
        };

        crate::process_csv(&self.input,output,self.format);
    }
}


fn parse_format(format:&str) ->Result<OutputFormat,anyhow::Error> {
    format.parse();
}

impl From<OutputFormat> for &'static str{
    fn from(format:OutputFormat) -> Self{
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}


impl FromStr for OutputFormat {

    type Err = anyhow::Error;

    fn from_str(s:&str) -> Result<Self,Self::Err> {
       
            match s {
                "json" => Ok(OutputFormat::Json),
                "yaml" => Ok(OutputFormat::Yaml),
                _ => Err(anyhow::anyhow!("Invalid format")),
            
        }
    }
    
}


impl fmt::Display for OutputFormat {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_'>) -> fmt::Result {
        write!(f,"{}",Into::<&str>::info(*self);)
    }
}






