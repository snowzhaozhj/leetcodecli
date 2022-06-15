# Leetcode Cli

一个用来刷leetcode的命令行工具。

```shell
leetcodecli 0.1.0

USAGE:
    leetcodecli.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    auth      login or logout
    help      Print this message or the help of the given subcommand(s)
    list      list problems
    pick      pick a problem
    submit    submit your answer
```

目前支持以下功能: 

* auth: 使用cookie登录leetcode
* list: 列出所有问题
* pick: 选择一个问题来回答
* submit: 提交答案，或者测试答案

TODO:

* 支持github登录
* 根据用户指定的一些参数来过滤问题

