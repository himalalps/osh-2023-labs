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
需要对`args`进行错误判断，假如有多余的参数，需要报错.同时注意到`~`的扩展也是在shell中进行的，需要利用`getenv`函数获取`HOME`环境变量的值，同时假如`getenv("HOME")`失败，也需要进行错误处理，一种方式是使用`getpwuid`这样返回的`passwd`结构体中的`pw_dir`对应的就是当前用户的家目录.

3. `cd`无参数，直接返回`home`，获取`home`方式见2.

## 管道

1. 

## 拓展功能

1. 支持`$`扩展，获取环境变量，使用了`getenv`函数.