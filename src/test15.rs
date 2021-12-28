pub type BoxedError =Box<dyn Error +Send + Sync>;

pub trait Executor{
    fn run(&self)->Result<Option<i32>,BoxedError>;
}

pub struct Shell<'a,'b>{
    cmd:&'a str,
    args:&'b[&'a str],
}
//shell的构造方法
impl<'a,'b> Shell<'a,'b> {
    pub fn new(cmd:&'a str, args:&'b[&'a str]) -> Self{
        Self{cmd, args}
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>,BoxedError>{
        let output= Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

//使用泛型参数
pub fn executor_generics(cmd:&impl::Executor)->Result<Option<i32>,BoxedError>{
    cmd.run()
}

// 使用 trait object: &dyn T
pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

//使用trait object
pub fn executor_boxed_trait_object(cmd:Box<dyn Executor>) -> Result<Option<i32>,BoxedError>{
    cmd.run()
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn shell_shall_work() {
        let cmd = Shell::new("ls", &[]);
        let result = cmd.run().unwrap();
        assert_eq!(result, Some(0));
    }
    #[test]
    fn execute_shall_work() {
        let cmd = Shell::new("ls", &[]);
        let result = execute_generics(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let result = execute_trait_object(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let boxed = Box::new(cmd);
        let result = execute_boxed_trait_object(boxed).unwrap();
        assert_eq!(result, Some(0));
    }
}