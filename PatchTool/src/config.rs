use std::string::String;
use std::io;
use std::fs;
use std::path::Path;

pub struct Config
{
    pub windows_bin_loc: String,
    pub linux_bin_loc: String,
    pub osx_bin_loc: String
}

pub fn load_config() -> io::Result<Config>
{
    let data = fs::read_to_string(Path::new("./config.json"))?;
    match json::parse(&data)
    {
        Ok(j) =>
        {
            let wb = j["Win64"].as_str();
            let lb = j["Linux"].as_str();
            let mb = j["OSX"].as_str();

            if wb == None
            {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing property Win64"));
            }
            if lb == None
            {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing property Linux"));
            }
            if mb == None
            {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing property OSX"));
            }
            return Ok(Config
            {
                windows_bin_loc: String::from(wb.unwrap()),
                linux_bin_loc: String::from(lb.unwrap()),
                osx_bin_loc: String::from(mb.unwrap())
            });
        },
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
    }
}
