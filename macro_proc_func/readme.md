## Run
```
Cargo test --test builder
```

## Notice
1. 使用了trybuild
2. trybuild使用了pass和compile_fail
3. cargo test会自动编译tests下所有文件，会导致fail.rs不经过trybuild框架就出错
    * 需要在根Cargo.toml中，配置builder的tests入口，避免全部编译
    * 运行时使用Cargo test --test 入口
4. fail.rs第一次运行时会失败，同时在wip目录下生成错误结果,把对应的stderr错误文件拷贝到和fail.rs同目录，再次运行，
trybuild会比较fail.rs的输出和stderr文件，成功则测试通过
