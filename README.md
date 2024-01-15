## 简单测试 luajit, c 函数运行效率

运行: 

```bash
cargo bench
```
## 使用方法

在 [lua/export.lua](./lua/export.lua) 中添加需要测试的 lua 实现

在 [c/export.c](./c/export.c) 中添加需要测试的 c 实现

在 [src/lib.rs](./src/lib.rs) 中添加需要测试的 rust 实现, **并且导出 c 方法**

在 [benches/bench.rs](./benches/bench.rs) 中添加需要测试的方法

## 运行环境

硬件: MacBook Pro M1, 16G

### case 1. 创建一个 luajit 实例

```txt
lua vm create           time:   [44.763 µs 45.017 µs 45.269 µs]
```

### case 2. 简单函数调用

1. 运行简单 lua 函数 (lua.export.add)

```txt
lua.export.add(1,1)     time:   [36.704 ns 36.951 ns 37.243 ns]
```

2. 运行简单 c 函数 (c/export.c#add)

```txt
c.add(1+1)              time:   [952.50 ps 957.62 ps 963.68 ps]
```

### case 3. fibonacci 函数调用

1. 运行 lua 版本 (lua.export.fib(10))

```txt
lua.export.fib(10)      time:   [441.49 ns 442.43 ns 443.62 ns]
```

2. 运行 lua 调用 c 版本 (lua.export.fib_call_c)

```txt
globals.fib_call_c(1,1) time:   [210.19 ns 211.14 ns 212.28 ns]
```

3. 运行 c 版本 (c/export.c#fib)

```txt
c.fib(10)               time:   [159.48 ns 160.29 ns 161.21 ns]
```

4. 运行 rust 版本 (src/lib.rs#fib)

```txt
rust.fib(10)            time:   [159.33 ns 159.82 ns 160.37 ns]
```

