# 实验一：编译和裁剪 Linux 内核

## Linux 编译

按文档，完成必要依赖的下载与更新后，解压linux 6.2.7内核，执行以下操作即可完成编译.
```shell
make defconfig # 创建默认配置
make menuconfig # 打开配置修改界面
make -j $((`nproc`-1)) # 使用多核并行编译内核
```
不进行修改直接编译，得到的`bzImage`镜像较大，且编译花费时间较多，大约花费近二十分钟才得到结果，使用`ls -l`指令可以看出其大小约12MiB.
```shell
$ ls -l ./arch/x86/boot/bzImage 
-rw-r--r-- 1 hao hao 12426432 Mar 19 10:05 arch/x86/boot/bzImage
```
因此，需要对内核进行裁剪。在`menuconfig`界面中，每个选项都有对应的help选项，查看该选项可以对一些选项选择Y(包含)和N(去除)，或是指定一些值的大小，依次探索，就可以将内核大小裁剪变小。但也存在一些关键的选项去除之后会导致之后挂载启动盘时出现kernel panic的情况，所以在裁剪一些内容之后，我会进行测试是否影响后续内容的进行。

测试发现，一级菜单中除去`64-bit kernel`选项外均可以去除，同时存在一些选项可以加快编译的时间，如`general setup`中的`kernel compression mode`选项，修改这一选项可以调整内核压缩模式，影响压缩时间和压缩后的大小。同时以下这些选项也可以去除：help菜单中提示if unsure, choose 'N'；没有必要的，比如LED，声卡等驱；与所使用平台无关的选项，如mac等；支持远程的选项等等。

最终得到的内核镜像大小如下
```shell
$ ls -l ./arch/x86/boot/bzImage 
-rw-r--r-- 1 hao hao 3856912 Mar 19 11:55 arch/x86/boot/bzImage
```

## 创建初始内存盘

去除`while(1) {}`后再使用QEMU测试时，会提示kernel panic，

## 添加自定义系统调用

