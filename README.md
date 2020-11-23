# Rust by Example

要做到熟练运用，还是得敲代码

### Day 1 [2020-11-16]:

- [p001_debug](./src/p001_debug.rs) 使用 #[derive(Debug)]
- [p002_impl_display](./src/p002_impl_display.rs) 实现 std::fmt::Display trait，在 println 时使用 {}
- [p003_formatting](./src/p003_formatting.rs) 一些格式化操作，例如 0x{:02X}
- [p004_tuples](./src/p004_tuples.rs) 关于元组
- [p005_array_slices](./src/p005_array_slices.rs) 数组和切片

### Day 2 [2020-11-17]

- [p006_struct](./src/p006_struct.rs) 各种类型结构体，构造，解构
- [p007_enum](./src/p007_enum.rs) 枚举的定义，match 匹配，类型别名
- [p008_enum_linked_list](./src/p008_enum_linked_list.rs) 使用枚举构建链表，use 的使用，box，递归
- [p009_constants](./src/p009_constants.rs) 常量的定义，static 和 const
- [p010_aliasing](./src/p010_aliasing.rs) 数据类型别名，类型的强制转换

### Day 3 [2020-11-20]

- [p011_from_into](./src/p011_from_into.rs) 类型之间的转换
- [p012_tryfrom_tryinto](./src/p012_tryfrom_tryinto.rs) 尝试类型之前的转换，处理有可能出错的情况
- [p013_tostring_fromstring](./src/p013_tostring_fromstring.rs) 某个类型，转换成字符串，将字符串，转换成某个类型

### Day 4 [2020-11-23]

- [p014_if](./src/p014_if.rs) if 语法
- [p015_loop](./src/p015_loop.rs) loop 语法，loop 标签
- [p016_while](./src/p016_while.rs) while 语法
- [p017_for_and_range](./src/p017_for_and_range.rs) for in 语法，for in iter、into_iter、iter_mut 的使用
