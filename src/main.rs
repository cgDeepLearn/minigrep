use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //let args: Vec<String>= env::args().collect();
    
    //unwrap_or_else 定义于Result<T, E>上，Ok时返回Ok内部封装的值，
    // 当其值是Err时 该方法会调用一个 闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    // println!("args are {:?}", args);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }


    // let contents = fs::read_to_string(&config.filename)
    //     .expect("Something went wrong reading the file");
    // println!("{} With Contents: {}", config.filename, contents);
    
}


