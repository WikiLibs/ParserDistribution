use clap::clap_app;
use std::path::Path;
use std::io;
use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;

mod config;

fn check_api_key(buf: &Vec<u8>, i: usize) -> bool
{
    if i + 37 >= buf.len()
    {
        return false;
    }
    for j in 0..36
    {
        if buf[i + j] != 0x41 //Rust cannot consider a character as a single byte unlike C
        {
            return false;
        }
    }
    return true;
}

fn check_user(buf: &Vec<u8>, i: usize) -> bool
{
    if i + 37 >= buf.len()
    {
        return false;
    }
    for j in 0..36
    {
        if buf[i + j] != 0x55 //Rust cannot consider a character as a single byte unlike C
        {
            return false;
        }
    }
    return true;
}

fn patch_vec<T>(buf: &mut Vec<T>, start: usize, replacement: &[T])
    where T: Copy
{
    for i in 0..replacement.len()
    {
        buf[i + start] = replacement[i];
    }
}

fn write_binary(input: &Path, output: &Path, key: &[u8], user: &[u8]) -> io::Result<()>
{
    let mut buf: Vec<u8> = Vec::new();
    let mut fin = File::open(input)?;
    let mut fout = File::create(output)?;

    fin.read_to_end(&mut buf)?;
    for i in 0..buf.len()
    {
        if check_api_key(&buf, i)
        {
            patch_vec(&mut buf, i, key);
            buf[i + key.len()] = 0x0;
        }
        else if check_user(&buf, i)
        {
            patch_vec(&mut buf, i, user);
            buf[i + user.len()] = 0x0;
        }
    }
    fout.write(&buf)?;
    return Ok(());
}

fn main() {
    let matches = clap_app!(bin2c =>
        (version: "1.0")
        (author: "Yuri6037")
        (about: "The wikilibs parser distribution patch tool")
        (@arg OS: +required "The operating system name (Win64, Linux or OSX")
        (@arg API_KEY: +required "The api key to write")
        (@arg USER_UUID: +required "The user UUID to write")
    ).get_matches();
    let cfg = match config::load_config()
    {
        Ok(v) => v,
        Err(e) =>
        {
            eprintln!("Could not load config: {}", e);
            std::process::exit(1);
        }
    };
    let os = matches.value_of("OS").unwrap();
    let key = matches.value_of("API_KEY").unwrap();
    let user = matches.value_of("USER_UUID").unwrap();
    let output;
    let input;

    #[cfg(debug_assertions)]
    println!("Loaded config: Win64='{}', Linux='{}', OSX='{}'", cfg.windows_bin_loc, cfg.linux_bin_loc, cfg.osx_bin_loc);
    if os == "Win64"
    {
        output = Path::new("/tmp/wikilibs_parser_windows.exe");
        input = Path::new(&cfg.windows_bin_loc);
    }
    else if os == "Linux"
    {
        output = Path::new("/tmp/wikilibs_parser_linux");
        input = Path::new(&cfg.linux_bin_loc);
    }
    else if os == "OSX"
    {
        output = Path::new("/tmp/wikilibs_parser_osx");
        input = Path::new(&cfg.osx_bin_loc);
    }
    else
    {
        eprintln!("Unsupported OS name '{}'", os);
        std::process::exit(1);
    }
    match write_binary(input, output, key.as_bytes(), user.as_bytes())
    {
        Ok(()) =>
        {
            println!("{}", output.display());
            std::process::exit(0);
        },
        Err(e) =>
        {
            eprintln!("Could not patch binary: {}", e);
            std::process::exit(1);
        }
    }
}
