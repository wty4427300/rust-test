//开始看一下rust的io部分

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
//友好的错误提示

use std::*;

impl Config{
    fn new(args:&[String])->Result<Config,&'static str>{
        if args.len() > 3 {
            return Err("参数过多了");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive =env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename,case_sensitive })
    }
}

//dyn动态的意思 dynamic,?会抛出异常让调用者使用
pub fn io_test(config:Config)->Result<(),Box<dyn Error>>{
    //打开文件按照字符流打印
    let contents = fs::read_to_string(config.filename)?;

    println!("文件内容:\n{}", contents);

    Ok(())
}

//这个拆分出的逻辑是那个参数该放进那个变量
fn parse_config(args:&[String])->Config{
    let query = args[1].clone();
    let filename = args[2].clone();
    Config{query, filename,case_sensitive:true}
}

//在一个长串里查找一个子串
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    //将query转小写
    let query=query.to_lowercase();
    let mut result=Vec::new();
    for line in contents.lines()  {
        if line.to_lowercase().contains(&query){
            result.push(line)
        }
    }
    result
}

pub fn run(config:Config) ->Result<(),Box<dyn Error>>{
    //读取文件转成字符串
    let contents=fs::read_to_string(config.filename)?;

    let results=if config.case_sensitive{
        search_case_insensitive(&config.query,&contents);
    };

    for line in results{
        println!("{}",line);
    }
    Ok(())
}









