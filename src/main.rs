use clap::Parser;
use serde::{Serialize, Deserialize};
use std::{fs,fs::File, io::{BufReader, BufRead},cmp::{min}};

/* CLI struct */

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli{
    #[arg(short, long)]
   config_path: String,
}

/* Attack config YAML struct */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Item{
    list: Vec<String>,
    seperator: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
enum Category{
    Path(String),
    Range([i32;2]),
    Iterator(Vec<Item>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]

struct Request{
    target:String,
    raw:String,
    
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]

struct Position{
    tag:String,
    category: Option<String>,
    value: Category
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]

struct Payload{
    prefix: Option<String>,
    suffix: Option<String>,
    positions: Vec<Position>
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AttackConfig{
    name: String,
    delay: u16,
    request: Request,
    payload: Payload
}

/* YAML functions */
impl AttackConfig{

    fn start_attack(&self){
        /*
         * STEPS:
         * 1. positions & replace raw req:
         *      a. get positions data
         *      b. replace the raw req
         * 2. build req obj
         * 3. note the time
         * 4. Execute request
         * 5. Goto step 1
         */

        let min_len_items = self.payload.get_min_len();

        for index in 0..min_len_items{
            
        }
        // for position in self.payload.positions{
              
        // }

    }
}

impl Payload{
    fn get_min_len(&self)-> u32{
        let arr:Vec<u32> = self.positions.iter().map(
            |p|p.get_total_items()
        ).collect();

        *arr.iter().min().unwrap()
        
    }
}
impl Position {
    fn get_tag(&self)->String{
         format!("{{{}}}",self.tag )
    }
    fn get_total_items(&self)->u32{
        match &self.value {
            Category::Path(file_path) => {
                
                let file = File::open(file_path).expect(
                    &format!("Unable to open paylod file {}",file_path)
                );

                BufReader::new(file).lines().count() as u32
            },
            Category::Range(range) => range[1] as u32 - range[0] as u32,
            Category::Iterator(_) => 0,
        }
    }
    
}


/* entry point */
fn main() {

    let args = Cli::parse();

    let contents = fs::read_to_string(args.config_path);

    if contents.is_err() {
        return println!("{:?}",contents.err().unwrap().to_string());
    }

    let yaml = serde_yaml::from_str::<AttackConfig>(contents.unwrap().as_str());
    
    // let client = reqwest::Client::new();

    // client.request(method, url);
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut http_parser = httparse::Request::new(&mut headers);
    let parsed_req = http_parser.parse(yaml.as_ref().unwrap().request.raw.as_bytes());
    
    if parsed_req.unwrap().is_partial(){
        return println!("Invalid request raw body");
    }

    // parsed_req.unwrap().is_complete();
    // let file = fs::File::open(args.config_path).unwrap();
    // let reader = BufReader::new(file);
    // let yaml:Result<AttackConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);

    
    println!("Contents {:?}",yaml);
}
