
use mps_tiny_http::Serve as tinyServe;
use mps_tiny_http::request::Request;


pub struct Router<'a>{
    middleware_vec:Vec<Box<dyn Fn(&mut Request)+'a>>
}
impl<'a> Router<'a>{
    pub fn new()->Self{
        Self { middleware_vec: Vec::new() }
    }
    fn create_router(method:&'a str,url:&'a str,response:&'a str)->Box<dyn Fn(&mut Request)+'a>{
        Box::new(move |req:&mut Request|{
            if req.method().as_str() == method && req.url().as_str()  == url{
                req.response(response);
            }
        })
    }
    pub fn get(&mut self,url:&'a str,response:&'a str){
        let m_fn = Self::create_router("GET", url, response);
        self.middleware_vec.push(m_fn)
    }
    pub fn post(&mut self,url:&'a str,response:&'a str){
        let m_fn = Self::create_router("POST", url, response);
        self.middleware_vec.push(m_fn)
    }
    pub fn delete(&mut self,url:&'a str,response:&'a str){
        let m_fn = Self::create_router("DELETE", url, response);
        self.middleware_vec.push(m_fn)
    }
    pub fn patch(&mut self,url:&'a str,response:&'a str){
        let m_fn = Self::create_router("PATCH", url, response);
        self.middleware_vec.push(m_fn)
    }

    
}
pub struct Serve<'a>{
    router:Router<'a>

}
impl Serve<'_>{
    pub fn new()->Self{
        Self { router: Router::new()}
    }
    pub fn run(&self,addr:&str){
        let serve = tinyServe::new(addr);
        serve.connect();
        for mut res in serve.incoming_requests(){
            for f in &self.router.middleware_vec{
                f(&mut res);
            }
        }
    }
}


mod test{
    #[cfg(test)]
    use super::*;
    #[test]
    fn test_add_middleware(){
        let mut router = Router::new();
        router.get("/get", "6666");
    }
    #[test]
    fn test_serve(){
        let mut serve = Serve::new();

        //添加路由
        serve.router.get("/get", "cascascascasc");
        
        //启动服务器
        serve.run("127.0.0.1:8888")
    }
}