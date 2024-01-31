use clap::Parser;
use bigtools::BigWigRead;
use bigtools::utils::remote_file::RemoteFile;
use std::fmt;

struct Region {
    chrom_name: String,
    start: u32,
    end: u32,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL for the BigWig file
    #[arg(short, long)]
    url: String,

    /// Genomic region
    #[arg(short, long)]
    region: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let f = RemoteFile::new(&args.url);

    let region = parse_region(&args.region)?;

    let mut reader = BigWigRead::open(f)?;
    //match reader.get_interval(&region.chrom_name, region.start, region.end) {
    //    Ok(intervals) => {
    //        for interval in intervals {
    //            match interval {
    //                Ok(value) => {
    //                    println!("{}\t{}", value.start, value.value);
    //                },
    //                Err(e) => {
    //                    eprintln!("Error: {}", e);
    //                },
    //            }
    //        }
    //    },
    //    Err(e) => {
    //        eprintln!("Error: {}", e);
    //    },
    //}

    let values = reader.values(&region.chrom_name, region.start, region.end)?;

    let mut pos = region.start;
    for value in values {
        println!("{}\t{:.1}", pos, value);
        pos += 1;
    }
    
    Ok(())
}

#[derive(Debug)]
struct ParseError {
    msg: String,
}

impl std::error::Error for ParseError {
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

fn parse_region(regions_str: &str) -> Result<Region, ParseError> {

    let parts: Vec<&str> = regions_str.split(":").collect();

    let chrom_name = parts[0].to_string();

    let parts: Vec<&str> = parts[1].split("-").collect();

    let start;
    match parts[0].to_string().parse::<u32>() {
        Ok(s) => {
            start = s;
        },
        Err(_) => {
            return Err(ParseError{
                msg: "Error parsing region start".to_string(),
            })
        }
    }

    let end;
    match parts[1].to_string().parse::<u32>() {
        Ok(e) => {
            end = e;
        },
        Err(_) => {
            return Err(ParseError{
                msg: "Error parsing region end".to_string(),
            })
        }
    }

    Ok(Region{
        chrom_name,
        start,
        end,
    })
}
