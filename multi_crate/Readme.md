## 项目中包含：
1. 一个本地lib：mylib
2. 一个crate:server
3. 一个sub project: client

## 注意事项
1. mylib是独立的project，不受multi_crate/Cargo.toml控制
2. server受multi_crate/Cargo.toml控制，不受server/Cargo.toml控制。
   server/Cargo.toml是编译器生成的，可删除。
3. client是独立的project，不受multi_crate/Cargo.toml控制