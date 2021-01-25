//开始看一下rust的io部分

pub struct Config {
    query: String,
    filename: String,
}
//友好的错误提示
impl Config{
    fn new(args:&[String])->Result<Config,&'static str>{
        if args.len() > 3 {
            return Err("参数过多了");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
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
    Config{query, filename}
}