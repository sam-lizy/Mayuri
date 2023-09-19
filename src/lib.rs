
use std::collections::HashMap;
use mps_tiny_http::common::ContentType;
use mps_tiny_http::Serve as tinyServe;
use mps_tiny_http::request::Request;


pub struct Router<'a>{
    middleware_vec:Vec<Box<dyn Fn(&mut Request)+'a>>,
    headers:Option<HashMap<String,String>>
}
impl<'a> Router<'a>{
    pub fn new()->Self{
        Self { 
            middleware_vec: Vec::new(),
            headers:None
         }
    }
    fn create_router(&mut self,method:&'a str,url:&'a str,response:&'a str)->Box<dyn Fn(&mut Request)+'a>{
        let mut header:HashMap<String, String> = HashMap::new();
        if let Some(headers) =  &self.headers{
            header = headers.clone();
        };
        let fn_box = Box::new(move |req:&mut Request|{
            if req.method().as_str() == method && req.url().as_str()  == url{
                for (k,v) in header.iter(){
                    req.response.set_headers(k, &v);
                }
                req.response(response);
            }
        });
        self.headers = None;
        return fn_box;
        
    }
    pub fn set_contentType(&mut self,ty:ContentType){
        let content_str:&str = ty.into();
        if let Some(headers) = &mut self.headers {
            headers.insert(String::from("Content-Type"), String::from(content_str));
        }else {
            let mut hm = HashMap::new();
            hm.insert(String::from("Content-Type"), String::from(content_str));
            self.headers = Some(hm)
        }
    }
    pub fn set_headers(&mut self,key:&str,value:&str)->&mut Self{
        self.headers = Some(HashMap::new());
        self.headers.as_mut().unwrap().insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn get(&mut self,url:&'a str,response:&'a str)->&mut Self{
        let m_fn = self.create_router("GET", url, response);
        self.middleware_vec.push(m_fn);
        self
        
    }
    pub fn post(&mut self,url:&'a str,response:&'a str){
        let m_fn = self.create_router("POST", url, response);
        self.middleware_vec.push(m_fn)
    }
    pub fn delete(&mut self,url:&'a str,response:&'a str){
        let m_fn = self.create_router("DELETE", url, response);
        self.middleware_vec.push(m_fn)
    }
    pub fn patch(&mut self,url:&'a str,response:&'a str){
        let m_fn = self.create_router("PATCH", url, response);
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
        serve.router.set_contentType(ContentType::ApplicationJson);
        serve.router.get("/get", "{\"name\":lzy}");
        serve.router.set_headers("Authorization", "ioaopscoapso").get("/test", "success");
        //启动服务器
        serve.run("127.0.0.1:8888")
    }
}