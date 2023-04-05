# 实验二：简易shell

## 目录导航

1. `pwd`命令，使用`getcwd`函数即可，其函数原型为
```C
char* getcwd(char *buf, size_t size);
```
如果`size`过小或是获取目录失败，则返回`NULL`，否则返回`buf`.只需正确获得`buf`然后输出即可.

2. `cd`命令，使用`chdir`命令，其函数原型为
```C
int chdir(const char *path);
```
需要对`args`进行错误判断，假如有多余的参数，需要报错.同时注意到`~`的扩展也是在shell中进行的，需要利用`getenv`函数获取`HOME`环境变量的值，同时假如`getenv("HOME")`失败，也需要进行错误处理.