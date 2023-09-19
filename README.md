```rust
        let mut serve = Serve::new();

        //添加中间件
        serve.router.use_middleware(|req|{
            if req.url() == "tttt"{
                Some("casd")
            }else {
                None
            }

        });
        
        // 添加路由
        serve.router.set_contentType(ContentType::ApplicationJson);
        serve.router.get("/get", "{\"name\":lzy}");

        serve.router.set_headers("Authorization", "ioaopscoapso").post("/comment", "cascascdgascasc");
        serve.router.patch("/commend", "cascfgascascasc");
        serve.router.delete("/reply", "casfg");
        //启动服务器
        serve.run("127.0.0.1:8888")

```

