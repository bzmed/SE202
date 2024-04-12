use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    value: u32,
    
    #[clap(short, long)]
    verbose: bool,
    
    #[clap(short = 'm', long, default_value_t = 0)]
    min: u32,
}


fn fibo(n: u32) -> Option<u32> {
    if n <= 1 {
        return Some(n);
    }

    let mut a:Option<u32> = Some(0);
    let mut b:Option<u32> = Some(1);

    for _ in 2..=n {
        let c = a.unwrap().checked_add(b.unwrap());
        a = b;
        b = c;
    }
    b
}

fn main() {
    let cli = Cli::parse();

    let Cli { value, verbose, min } = cli;

    for i in min..=value {
        match fibo(i) {
            Some(result) => {
                if verbose || i == value {
                    println!("fibo({})= {}", i, result);
                }
            },
            None => {
                println!("Value too large to compute");
                break;
            }
        }
    }
}


