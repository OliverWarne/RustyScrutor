extern crate rustc_serialize;

use std::string::String;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rustc_serialize::json;

// Automagically generates json encode & decode stuff
#[derive(RustcDecodable, RustcEncodable,Debug)]
struct Match {
    // Team info
    name                     : String,
    num                      : String,

    // Match info
    match_num                : usize,
    notes                    : String,

    // Match data
    stacked_height           : i8,
    bin_stacked_height       : i8,
    other_bin_stacked_height : i8,
    chute_used               : bool,
    alternate_platform       : bool,
    steal_bins               : bool,
    throw_noodles            : bool,
    fill_bin                 : bool,
    coop                     : bool,
    auto                     : bool,
    toteset                  : bool,
}

// Simple bool comparerator for config/matchses
fn comp(val: &bool, config: &bool) -> bool {
    if val == config {
        return true;
    } else if config == &false {
        return true;
    } else {
        return false;
    }
}

impl Match {
    // Grabs teamname + name. Used for filenames
    fn team_identity(&self) -> String {
        let mut result = String::new();
                     //"&" casts "String" into &str???? ok rust
        result.push_str(&self.name);
        result.push_str(" : ");
        result.push_str(&self.num);
        return result;
    }
    
    fn match_bool(&self) -> [bool; 11] {
        
        // This is the "config". It is basically the minimums of what the team
        // has to score.
        let config : Match = Match{name               : "".to_owned(),
                             num                      : "".to_owned(),

                             match_num                : 0,
                             notes                    : "".to_owned(),

                             stacked_height           : 6,
                             bin_stacked_height       : 0,
                             other_bin_stacked_height : 0,
                             chute_used               : false,
                             alternate_platform       : false,
                             steal_bins               : true,
                             throw_noodles            : true,
                             fill_bin                 : true,
                             coop                     : true,
                             auto                     : true,
                             toteset                  : false};

        let mut bool_array: [bool; 11] = [false; 11];

        // There has to be a more elegant solution...
        bool_array[0]  = &self.stacked_height >= &config.stacked_height;
        bool_array[1]  = &self.bin_stacked_height >= &config.bin_stacked_height; 
        bool_array[2]  = &self.other_bin_stacked_height >= &config.other_bin_stacked_height; 
        bool_array[3]  = comp(&self.chute_used,&config.chute_used);
        bool_array[4]  = comp(&self.alternate_platform,&config.alternate_platform);
        bool_array[5]  = comp(&self.steal_bins,&config.steal_bins);
        bool_array[6]  = comp(&self.throw_noodles,&config.throw_noodles);
        bool_array[7]  = comp(&self.fill_bin,&config.fill_bin);
        bool_array[8]  = comp(&self.coop,&config.coop);
        bool_array[9]  = comp(&self.auto,&config.auto);
        bool_array[10] = comp(&self.toteset,&config.toteset);

        return bool_array;
    }

    fn match_percentage(&self) -> usize {
        let match_array = &self.match_bool();
        let mut score = 0;

        for x in 0..match_array.len() {
            if match_array[x]  == true { score += 1;}
        }
        // float(f32) is used to prevent rounding while being divided (returning 0)
        let percentage: f32 = score as f32 / match_array.len() as f32 * 100f32;

        // This automatically rounds it.
        return percentage as usize;
    }
}

fn filedecode(filepath: &str) -> Match {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file)    => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => println!("Read the file!"),
    };

    let dec: Match = json::decode(&s).unwrap();

    return dec;
}

fn fileenc(mtch: &Match) {
    let mut filename: String  = mtch.team_identity();
    filename.push_str(".txt");
    // TODO : Not be such an idiot and figure how to remove whitespace.
    let v: Vec<&str> = filename.split_whitespace().collect();
    let finalpath = v.connect("");


    let path = Path::new(&finalpath);
    let display = path.display();
    
    let enc = json::encode(mtch).unwrap();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(&enc.as_bytes()) {
        Err(why) => {
            panic!("Couldn't write  {} : {}",
                            display,
                            Error::description(&why))
        },
        Ok(_) => println!("Wrote to {}", display),
    }
}

fn teamsfrompath(srcpath: &str) -> Vec<Match> {
    let paths = fs::read_dir(srcpath).unwrap();
    let mut teams: Vec<Match> = Vec::<Match>::new();
    
    for rawpath in paths {
        let path = rawpath.unwrap().path();
        //println!("Name: {}", &path.display());
        let pathstr: &str = path.to_str().unwrap(); 
        
        if pathstr.ends_with(".txt") {
            //debug
            //println!("WORKED: {}", &pathstr);
            let matchpath: Match = filedecode(&pathstr);
            &teams.push(matchpath);
        }
    }
    return teams;
}

fn scanprocess(textpath: &str) -> isize {
    let matches: Vec<Match> = teamsfrompath(textpath);
    for matcho in matches {
        println!("{}",matcho.team_identity());
        println!("{}",matcho.match_percentage());
    }
    return 1isize;
}

fn main () {
    scanprocess("/home/oliver/prog/rust/scrutor/");
}
