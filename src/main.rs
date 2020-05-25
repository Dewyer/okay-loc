//!
//! To use just install it `cargo install okay-loc`
//!
//! Then run `okay-loc --help` for available flags.
//!
//! OR you can try running `okay-loc -p ./` to count the lines of code in the current directory
//!

pub mod exclusion;
pub mod loc_result;

use clap::{Arg, App,ArgMatches};
use std::path::{Path,PathBuf};
use regex::Regex;
use std::ops;
use std::ops::Add;
use std::fs;
use loc_result::LocResult;
use exclusion::Excludor;
use std::fs::DirEntry;

pub fn parse_exclude_regex(app_args:&ArgMatches) -> Result<Option<Regex>,anyhow::Error>
{
    let exclude_str_opt = app_args.value_of("exclude");
    Ok(match exclude_str_opt
    {
        Some(exclude_str) =>
        {
            let regex = Regex::new(exclude_str)?;
            Some(regex)
        },
        None=>None
    })
}

pub fn parse_start_path(app_args:&ArgMatches) -> Result<PathBuf,anyhow::Error>
{
    let path_str = app_args.value_of("path").ok_or_else(|| anyhow::Error::msg("You must give a path. (Use -p)"))?;
    Ok(Path::new(path_str).to_owned())
}

pub fn run_for_file(path:&PathBuf,details:bool) -> Result<LocResult,anyhow::Error>
{
    if (details) {
        print!("Checking file : {} : ", path.to_str().expect("pls path"));
    }
    let mut res = LocResult::new();
    let file_contents = fs::read_to_string(path)?;
    let file_lines:Vec<&str> = file_contents.split("\n").collect::<Vec<&str>>();
    for line in file_lines
    {
        if line.chars().count() == 0
        {
            continue;
        }

        if line.chars().all(|cc|cc.is_whitespace())
        {
            continue;
        }

        if line.contains("TODO")
        {
            res.todo_count +=1;
        }

        if line.trim().starts_with("//")
        {
            res.comment += 1;
        }
        else
        {
            res.source += 1;
        }
    }
    if (details)
    {
        println!("({} lines)",res.source)
    }
    Ok(res)
}

pub fn run_for_directory(path:PathBuf,excludor:&mut Excludor,details:bool) -> Result<LocResult,anyhow::Error>
{
    let elements = fs::read_dir(path)?;
    let mut res = LocResult::new();
    let mut non_ignores:Vec<DirEntry> = Vec::new();
    for dir_el in elements
    {
        let dir_el = dir_el?;
        if dir_el.file_name() == ".gitignore"
        {
            excludor.append_gitignore(&dir_el.path());
        }
        else
        {
            non_ignores.push(dir_el);
        }
    }

    for dir_el in non_ignores.iter()
    {
        let meta = dir_el.metadata()?;
        let dir_el_path = dir_el.path();
        if !excludor.is_file_included(&dir_el_path)
        {
            if details {
                println!("Skipped file, {:?}", dir_el_path.to_str());
            }
            continue;
        }

        if meta.is_dir()
        {
            let sub_res = run_for_directory(dir_el_path.clone(),excludor,details)?;
            res = res + sub_res;
        }
        else if meta.is_file()
        {
            let sub_res_res = run_for_file(&dir_el_path,details);
            match sub_res_res
            {
                Ok(sub_res)=>res = res + sub_res,
                Err(ee)=>{}
            }
        }
    }

    Ok(res)
}

pub fn run_loc(app_args:&ArgMatches) -> Result<(),anyhow::Error>
{
    let exclude = parse_exclude_regex(&app_args)?;
    let path = parse_start_path(&app_args)?;
    let details_flag = app_args.occurrences_of("details") > 0;

    let mut excludor = exclusion::Excludor::new(exclude,details_flag);
    let final_res = run_for_directory(path,&mut excludor,details_flag)?;
    println!("===========");
    println!("   All : {}",final_res.get_all());
    println!("   Source: {}",final_res.source);
    println!("   Comments: {}",final_res.comment);
    println!("   TODOs: {}",final_res.todo_count);
    println!("Nice !!");
    Ok(())
}

fn main() {
    let matches = App::new("Okay-Loc")
        .version("0.1.0")
        .author("Barnabás Rátki <barna@wexo.systems>")
        .about("Okay way of getting lines of code")
        .arg(Arg::with_name("exclude")
            .short("e")
            .long("exclude")
            .takes_value(true)
            .help("Regex to exclude extra files"))
        .arg(Arg::with_name("details")
            .short("d")
            .long("details")
        )
        .arg(Arg::with_name("path")
            .required(true)
            .short("p")
            .long("path")
            .takes_value(true)
            .help("Where to start the scan from"))
        .get_matches();
    println!("Counting lines of code!");

    let run_res = run_loc(&matches);
    match run_res
    {
        Ok(res)=>{},
        Err(ee)=>{
            println!("F dude: {:?}",ee);
        }
    }
}
