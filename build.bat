@echo off
REM Rust 项目构建脚本
REM 解决 MSVC 链接器找不到系统库的问题

setlocal

REM 设置库文件搜索路径
set "LIB=D:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22621.0\ucrt\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22621.0\um\x64"

REM 编译发布版本
cargo build --release

endlocal
