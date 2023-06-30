use chrono::Local;
use clap::Parser;
use std::fs::File;
use std::io::{Write, Read};
use std::fs::OpenOptions;
use std::io::{self, BufRead};

pub mod files;
pub mod utilities;
#[derive(Parser, Debug, Clone)]
pub struct CliArg {
    #[arg(short, long)]
    pub out: String,
    #[arg(short, long, default_value_t = String::from("100M"))]
    pub size: String,
    #[arg(short, long, default_value_t = 20)]
    pub keep: u32,
    #[arg(short, long, default_value_t = false)]
    pub dated:bool,
    #[arg(long, default_value_t = false)]
    pub crlf:bool
}

pub struct Config {
    pub out: String,
    pub size: u64,
    pub keep: u32,
    pub dated: bool,
    pub written: u64,
    pub active_fh: io::BufWriter<File>,
    pub crlf:bool
}

fn main() {
    let args = CliArg::parse();
    let size_limit_o = utilities::parse_size(&args.size);
    match size_limit_o {
        Err(cause) => {
            println!("{cause}");
            return;
        }
        _ => {}
    }
    let size_limit = size_limit_o.unwrap();
    let out_file = args.out;
    let dated = args.dated;
    let keep = args.keep;
    let mut written: u64 = 0;
    let current_size = files::path_get_size(&out_file);
    if let Ok(some) = current_size {
        written = some;
    }

    let active_fh = open_file(&out_file.clone()).expect("Can't open out file");
    let active_fh = io::BufWriter::new(active_fh);
    let crlf = args.crlf;
    let mut config = Config {
        out: out_file,
        size: size_limit as u64,
        keep,
        dated,
        written,
        active_fh,
        crlf
    };
    run(&mut config);
}


fn run(config:&mut Config) {
    if config.dated {
        pipe_text(config);
    } else {
        pipe_binary(config);
    }
}

fn pipe_text(config:&mut Config) {
    for i in io::stdin().lock().lines() {
        let line = i.unwrap();
        if config.dated {
            let ts = time_as_str();
            let mut data: String = format!("[{}] ", ts);
            data.push_str(&line);
            if config.crlf {
                data.push_str("\r\n");
            } else {
                data.push_str("\n");
            }

            append_data(config, data.as_bytes());
        } else {
            let mut data: String = String::from("");
            data.push_str(&line);
            if config.crlf {
                data.push_str("\r\n");
            }
            else {
                data.push_str("\n");
            }
            append_data(config, data.as_bytes());
        }
    }
}

fn time_as_str() -> String {
    let current_time = Local::now();
    let formatted_time = current_time.to_rfc3339();
    return formatted_time
}

fn pipe_binary(config:&mut Config) {
    let mut buffer = [0u8; 256 * 1024];
    let mut locked_stdin = io::stdin().lock();
    loop {
        let size_r = locked_stdin.read(&mut buffer);
        match size_r {
            Err(cause) => {
                let code = cause.raw_os_error();
                match code {
                    Some(i) => {
                        if i == 4 {
                            continue;
                        }
                        break;
                    },
                    None => {
                        break;
                    }
                }
            },
            Ok(size) =>{
                if size == 0 {
                    // means end of file reached at stdin
                    break;
                }
                append_data(config, &buffer[..size]);
            }
        }
    }
}

fn append_data(config :&mut Config, buffer:&[u8]) {
    check_rotate(config, buffer.len());
    let write_result = config.active_fh.write(buffer).expect("Write unsuccessful");
    config.written += write_result as u64;
}

fn check_rotate(config:&mut Config, read_count:usize) {
    if config.written > 0 && (config.written + read_count as u64 > config.size) {
        rotate_files(config);
    }
}

fn rotate_files(config:&mut Config) {
    for i in (1..config.keep).rev() {
        let current_file = format!("{}.{}", config.out.as_str(), i);
		let next_file  = format!("{}.{}", config.out.as_str(), i + 1);
        if files::path_exists(&current_file) {
            files::rename_file(&current_file, &next_file).expect("Rename file failed");
        }
	}
    let _ = config.active_fh.flush().expect("Failed to flush file");
    
    let target = format!("{}.1", config.out.as_str());
    files::rename_file(&config.out.as_str(), &target).expect("Rename file failed");
	let fh = open_file(config.out.as_str()).expect("Can't open file for writing");
    let fh = io::BufWriter::new(fh);
    config.active_fh = fh;
    config.written = 0;
}

fn open_file(file:&str)-> Result<File, std::io::Error> {
    if files::path_exists(file) {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(file)
    } else {
        File::create(file)
    }
}