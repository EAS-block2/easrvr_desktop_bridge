#![feature(proc_macro_hygiene, decl_macro)]
use std::collections::HashMap;
use serde_yaml;
use serde::Deserialize;
use std::{io::Read, fs::File};
#[macro_use] extern crate rocket;

#[get("/pc/<name>")]
fn retn(name: String) -> String {
    let mut g = false;
    let mut s = false;
    let mut to_return: String = "clear".to_string();
    let conf_f = std::fs::File::open("/etc/EAS/rktconfig.yaml").expect("Can't Find Config!");
    let config: Config = serde_yaml::from_reader(conf_f).expect("Bad YAML config file!");
    match config.pc_auths.get(&name){
        Some(auth) => {
            for i in auth.iter(){
                match config.alm_info.get(i){
                    Some(alarm) => {
                        println!("{:?}",read_status(alarm.get("file").unwrap()).unwrap());
                        if !read_status(alarm.get("file").unwrap()).unwrap().is_empty(){
                            match i.as_str(){
                            "general" => g=true,
                            "silent" => s=true,
                            _ =>(),}
                            println!("I'm getting the address of:{}", alarm.get("addr").unwrap().to_string());
                            to_return = alarm.get("addr").unwrap().to_string();}
                    }
                    None => ()
            }}}
        None => {to_return = String::from("unauthorized");}
}
if g && s {to_return = config.alm_info.get("both").unwrap().get("addr").unwrap().to_string()}
to_return
}

#[catch(404)]
fn not_found() -> String{
    String::from("unauthorized")
}

fn read_status(path: &str) -> std::io::Result<Vec<String>>{
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let yam: Vec<String> = serde_yaml::from_str(&content).unwrap();
    Ok(yam)
}

fn main() {
    rocket::ignite().register(catchers![not_found]).mount("/", routes![retn]).launch();
}
#[derive(Deserialize)]
struct Config{
    pc_auths: HashMap<String, Vec<String>>,
    alm_info: HashMap<String, HashMap<String, String>>,
}