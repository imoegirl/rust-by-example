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

### Day 5 [2020-11-25]

- [p018_match](./src/p018_match.rs) 使用 match 匹配
- [p019_match_destructuring_tuple](./src/p019_match_destructuring_tuple.rs) 使用 match 解构元祖
- [p020_match_destructuring_enum](./src/p020_match_destructuring_enum.rs) 使用 match 解构结构体
- [p021_match_pointer_ref](./src/p021_match_pointer_ref.rs) 使用 match 解构引用
- [p022_match_struct](./src/p022_match_struct.rs) 使用 match 解构结构体
- [p023_match_bind_variable](./src/p023_match_bind_variable.rs) 在 match 匹配中，将匹配到的范围值绑定到一个变量上
- [p024_if_let](./src/p024_if_let.rs) 使用 if let 简化 match
- [p025_while_let](./src/p025_while_let.rs) 使用 while let 简化循环中的 match

### Day 6 [2020-11-30]

- [p026_function_methods](./src/p026_function_methods.rs) 函数，impl

### Day 7 [2020-12-08]
- [p027_closure](./src/p027_closure.rs) 最基本的闭包
- [p028_closure_capture](./src/p028_closure_capture.rs) 闭包捕获外部变量的三种方式，Fn FnMut FnOnce
- [p029_closure_as_input](./src/p029_closure_as_input.rs) 将闭包作为函数的参数
- [p030_function_as_input](./src/p030_function_as_input.rs) 满足 trait 约束的函数，也可以作为函数参数
- [p031_closure_as_output](./src/p031_closure_as_output.rs) 闭包作为函数的返回值，也就是在函数中创造一个闭包，并返回
- [p032_higher_order_function](./src/p032_higher_order_function.rs) 高阶函数

### Day 8 [2020-12-10]
- [p033_mod_visibility](./src/p033_mod_visibility.rs) 模块的可见性
- [p034_struct_visibility](./src/p034_struct_visibility.rs) 结构体字段可见性
- [p035_use](./src/p035_use.rs) 使用 use
- [p036_super_self](./src/p036_super_self.rs) 使用 super 和 self 关键字，消除访问歧义
- [p037_split_file](./src/p037_split_file/mod.rs) 代码写在多个文件里

### Day 9 [2020-12-11]
- [p038_cfg](./src/p038_cfg.rs) 使用 cfg 控制编译
- [p038_generic](./src/p038_generic.rs) 泛型
- [p039_generic_function](./src/p039_generic_function.rs) 泛型函数
- [p040_generic_impl](./src/p040_generic_impl.rs) 泛型 impl
- [p041_generic_trait](./src/p041_generic_trait.rs) 泛型 trait
- [p042_trait_bound](./src/p042_trait_bound.rs) 泛型约束
- [p043_empty_bound](./src/p043_empty_bound) 空类型约束
- [p044_multiple_bounds](./src/p044_multiple_bounds.rs) 多重约束
- [p045_where_bound](./src/p045_where_bound.rs) 使用 where 语句约束
- [p046_newtype](./src/p046_newtype.rs) 自定义类型
- [p047_assoc_types](./src/p047_assoc_types.rs) 关联类型，type 写在 trait 里

### Day 10 [2020-12-15]
- [p048_ownership](./src/p048_ownership.rs) 所有权
- [p049_reference](./src/p049_reference.rs) 可变引用和不可变引用的使用
- [p050_ref.rs](./src/p050_ref.rs) ref 关键字在解构中的使用
- [p051_lifetime.rs](./src/p051_lifetime.rs) 生命周期的各种使用方式

### Day 11 [2020-12-17]
- [p052_trait](./src/p052_trait.rs) trait的实现
- [p053_trait_derive](./src/p053_trait_derive.rs) trait的派生
- [p054_trait_ops](./src/p054_trait_ops.rs) 运算符重载
- [p055_trait_drop](./src/p055_trait_drop.rs) drop销毁
- [p056_iterators](./src/p056_iterators.rs) 迭代器
- [p057_clone](./src/p057_clone.rs) clone的使用

### Day 12 [2020-12-28]
- [p058_panic](./src/p058_panic.rs) 主动 panic
- [p059_option_unwrap](./src/p059_option_unwrap.rs) unwrap 操作
- [p060_map](./src/p060_map.rs) 使用 Option 的 map 代替 match