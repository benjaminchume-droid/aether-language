use std::{env,fs};
use aether::run;

fn main(){
 let mut args=env::args().skip(1);
 match args.next().as_deref(){
  Some("run")=>{let path=args.next().unwrap_or_else(||{eprintln!("usage: aether run <file.ae>");std::process::exit(2)});match fs::read_to_string(&path).map_err(|e|e.to_string()).and_then(|s|run(&s)){Ok(out)=>if !out.is_empty(){println!("{out}")},Err(e)=>{eprintln!("error: {e}");std::process::exit(1)}}},
  Some("lex")=>{let path=args.next().unwrap();let s=fs::read_to_string(path).unwrap();match aether::lexer::lex(&s){Ok(t)=>for x in t{println!("{:?}",x)},Err(e)=>{eprintln!("error: {e}");std::process::exit(1)}}},
  _=>{println!("Aether compiler\n\nCommands:\n  aether run <file.ae>\n  aether lex <file.ae>");}
 }
}
