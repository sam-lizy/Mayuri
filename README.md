```rust
        let mut serve = Serve::new();

        //添加路由
        serve.router.get("/get", "cascascascasc");
        
        //启动服务器
        serve.run("127.0.0.1:8888")
```

