use clap::{Arg, App};

pub fn create_drop_cli_app() -> App<'static,'static> {
  App::new("drop")
    .version("0.1")
    .author("Bryan G. <bryan@bryan.codes>")
    .about("Screenshot & file upload tool with S3 support - http://github.com/gilbertw1/drop")
    .arg(Arg::with_name("file")
         .value_name("FILE")
         .help("Optional file to upload")
         .index(1))
    .arg(Arg::with_name("host")
         .long("host")
         .value_name("HOST")
         .help("Custom host")
         .takes_value(true))
    .arg(Arg::with_name("audio")
         .short("-a")
         .long("audio")
         .value_name("BOOL")
         .help("Audio enabled screencast")
         .takes_value(true)
         .possible_values(&["true", "false"])
         .default_value("false"))
    .arg(Arg::with_name("video")
         .short("v")
         .long("video")
         .help("Record video"))
    .arg(Arg::with_name("unique-length")
         .short("-u")
         .long("unique-length")
         .value_name("LENGTH")
         .help("Length of unique string used to create filenames")
         .takes_value(true))
    .arg(Arg::with_name("filename-strategy")
         .long("filename-strategy")
         .value_name("STRATEGY")
         .help("File upload naming strategy")
         .possible_values(&["exact", "append", "replace"])
         .takes_value(true))
    .arg(Arg::with_name("aws-key")
         .long("aws-key")
         .value_name("AWS_KEY")
         .help("AWS access key")
         .takes_value(true))
    .arg(Arg::with_name("aws-secret")
         .long("aws-secret")
         .value_name("AWS_SECRET")
         .help("AWS access secret")
         .takes_value(true))
    .arg(Arg::with_name("aws-bucket")
         .long("aws-bucket")
         .value_name("AWS_BUCKET")
         .help("S3 Bucket to upload to")
         .takes_value(true))
}
