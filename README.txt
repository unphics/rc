需要先准备CMake,XMake,VulkanSDK,llvm
建议直接用Scoop安装上面的依赖
按照config模板配置本地库目录


cargo常用操作(防止忘记版):
    cargo new xxx 创建一个新工程名字是xxx
    cargo build 构建工程
    cargo run 运行工程
    cargo run --example xxx 运行examples目录中的xxx示例文件(文件中有入口函数main)
    cargo build --target=x86_64-pc-windows-gnu