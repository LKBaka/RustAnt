# RustAnt

[![Rust](https://img.shields.io/badge/Rust-1.88+-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-GPL3-green.svg)](LICENSE)

**RustAnt** 是一个用 Rust 编写的编程语言解释器，实现了完整的编程语言功能栈，包括词法分析、语法分析、字节码编译和虚拟机执行。

## 🚀 特性

- **完整的语言实现**: 支持变量声明、函数定义、控制流等 正在编写面向对象和闭包
- **高性能**: 基于字节码虚拟机，支持快速执行 (并非快速 fib(35) 都要15秒 (release o3) )
- **类型系统**: 支持整数、浮点数、布尔值、字符串、数组等数据类型
- **REPL环境**: 交互式编程环境，支持实时代码执行

## ⚠️ 重要警告

> **会不会有 Breaking Changes?**: 因为本项目正处于非正式版的开发阶段 所以为了功能需要可能会有一些破坏性改变 请见谅orz

> **开发状态**: 这是一个正在积极开发中的项目，某些功能可能不稳定或未完成

> **性能说明**: 当前版本性能有限，复杂计算（如斐波那契数列）可能需要较长时间

> **兼容性**: 需要 Rust 1.88+ 和 Edition 2024，低版本可能无法编译

## 🏗️ 架构概览

RustAnt 采用经典的编译器架构设计，主要包含以下核心组件：

```
源码 → 词法分析器 → 语法分析器 → 抽象语法树 → 字节码编译器 → 虚拟机执行
```

### 核心模块

- **`lexer/`** - 词法分析器，将源码转换为Token流
- **`parser/`** - 语法分析器，构建抽象语法树(AST)
- **`ast/`** - 抽象语法树定义和表达式处理
- **`byte_code_vm/`** - 字节码编译器和虚拟机
- **`object/`** - 运行时对象系统 (别问为什么不用 Enum 问就是萌新时候写的代码现在改不掉了)
- **`runner/`** - 代码执行器 (文件执行和REPL)

## 📦 安装

### ⚠️ 前置要求

> **重要**: 请确保满足以下要求，否则编译可能失败

- Rust 1.88+ (Rust Edition 2024)
- Cargo

> 🚨 **版本要求**: 
> - 必须使用 Rust Edition 2024
> - 低版本 Rust 无法编译此项目

### 构建

```bash
# 克隆仓库
git clone https://github.com/LKBaka/RustAnt.git
cd RustAnt

# 构建项目
cargo build

# 发布构建
cargo build --release
```

## 🎯 使用方法

### 注: 
```
path/to/your/rust_ant: 你的解释器路径
path/to/your/file.ant: 欲执行的脚本文件的路径
```

### 文件执行

```bash
# 执行 .ant 文件

# 直接使用解释器本体
path/to/your/rust_ant --file path/to/your/file.ant

# 使用 cargo 
cargo run -- --file path/to/your/file.ant
```

### REPL 模式

```bash
# 启动交互式环境 (使用解释器本体)
path/to/your/rust_ant

# 启动交互式环境 (使用 cargo)
cargo run
```

## 📝 语言语法

### 基本语法

```ant
// 变量声明
let x = 42;
let message = "Hello, RustAnt!";

// 函数定义
func add(a, b) {
    return a + b;
}

// 条件语句
if x > 10 {
    print("x is greater than 10");
} else {
    print("x is 10 or less");
}

// 循环
while x > 0 {
    print(x);
    x = x - 1;
}
```

### 数据类型

- **整数**: `42`, `-17`
- **浮点数**: `3.14`, `-2.5`
- **布尔值**: `true`, `false`
- **字符串**: `"Hello, World!"`
- **数组**: `[1, 2, 3, 4]`
- **字典**: `{1: 2, 3: 4, 5: 6}`

### 面向对象编程

> ⚠️ **开发中**: 面向对象功能正在开发中，当前版本不支持 可暂时使用字典模拟

### 内置函数

> 🚨 **函数较少**: 当前内置函数较少
>
> **TestPrint**: (在print函数到来前) 用于输出内容的表达式，语法为 `TestPrint xxx;`
  
`print`: print函数! **用于将一个对象输出至控制台: print(1 + 2)**    
`len`: len函数! **用于获取一个列表的长度**    
`copy`: copy函数! **拷贝一个对象并返回该对象**  
`id`: id函数! **获取一个对象的id并返回**    
`now`: now函数! **返回从 UNIX 纪元到现在的秒数**   

## 🧪 测试

项目包含完整的测试套件：

```bash


# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --package rust_ant --lib

```

## 🔧 开发

### 项目结构

```
src/
├── lexer/           # 词法分析器
├── parser/          # 语法分析器
├── ast/             # 抽象语法树
├── byte_code_vm/    # 字节码编译器和虚拟机
├── object/          # 运行时对象系统
├── runner/          # 代码执行器
├── module_importer/   # 模块系统 (没写呢)
└── utils.rs         # 工具函数
```

### 如何添加新功能

1. **新语法**: 在 `parser/` 中添加解析函数
2. **新表达式**: 在 `ast/expressions/` 中定义新的表达式类型
3. **新操作**: 在 `byte_code_vm/` 中添加字节码指令
4. **新对象类型**: 在 `object/` 中实现新的对象类型 然后在 `obj_enum/` 中添加对应类型

### 调试

启用VM的调试功能：

```bash
cargo run --features debug
```

## 📊 性能

> ⚠️ **性能说明**: 当前版本性能有限，适合学习和实验用途

> 🚨 **当前性能限制**: 
> - 递归函数性能较差（如 fib(35) 需要约15秒 (release o3)）
> - 复杂计算建议使用其他语言
> - 适合简单的脚本和教学用途

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

> 💡 **提示**: 如果您不熟悉 Git 命令行，推荐使用 [GitHub Desktop](https://desktop.github.com/) 进行图形化操作

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 贡献指南

- 请确保代码符合项目的编码规范
- 新功能请添加相应的测试用例 (编译器中的函数相关测试可豁免)
- 提交信息请使用清晰的描述
- 如有疑问，欢迎在 [Issues](https://github.com/LKBaka/RustAnt/issues) 中讨论

## 📄 许可证

本项目采用 GPL-3.0 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

> **开源协议**: 本项目使用 GPL-3.0 许可证，这意味着：
> - 您可以自由使用、修改和分发代码
> - 如果您分发修改后的版本，必须同样使用 GPL-3.0 许可证
> - 必须公开源代码

## 🙏 致谢

感谢所有为 RustAnt 项目做出贡献的开发者和用户。  
感谢 《用Go语言自制编译器》 这本书的作者 索斯藤· 鲍尔(Thorsten Ball) 语言参考本书的部分内容 

## 📞 联系方式

- 项目主页: [GitHub Repository](https://github.com/LKBaka/RustAnt)
- 问题反馈: [Issues](https://github.com/LKBaka/RustAnt/issues)
- 讨论区: [Discussions](https://github.com/LKBaka/RustAnt/discussions)

---

**RustAnt** - 用 Rust 构建的现代编程语言解释器 🦀
