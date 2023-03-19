# 实验一：编译和裁剪 Linux 内核

## Linux 编译

按文档，完成必要依赖的下载与更新后，解压linux 6.2.7内核，执行以下操作即可完成编译.
```shell
$ make defconfig # 创建默认配置
$ make menuconfig # 打开配置修改界面
$ make -j $((`nproc`-1)) # 使用多核并行编译内核
```
不进行修改直接编译，得到的`bzImage`镜像较大，且编译花费时间较多，大约花费近二十分钟才得到结果，使用`ls -l`指令可以看出其大小约12MiB.
```shell
$ ls -l ./arch/x86/boot/bzImage 
-rw-r--r-- 1 hao hao 12426432 Mar 19 10:05 arch/x86/boot/bzImage
```
因此，需要对内核进行裁剪。在`menuconfig`界面中，每个选项都有对应的help选项，查看该选项可以对一些选项选择Y(包含)和N(去除)，或是指定一些值的大小，依次探索，就可以将内核大小裁剪变小。但也存在一些关键的选项去除之后会导致之后挂载启动盘时出现kernel panic的情况，所以在裁剪一些内容之后，我会进行测试是否影响后续内容的进行。

测试发现，一级菜单中除去`64-bit kernel`选项外均可以去除，同时存在一些选项可以加快编译的时间，如`general setup`中的`kernel compression mode`选项，修改这一选项可以调整内核压缩模式，影响压缩时间和压缩后的大小。同时以下这些选项也可以去除：help菜单中提示if unsure, choose 'N'；没有必要的选项，比如LED，声卡等驱动；与所使用平台无关的选项，如mac等；支持远程的选项等等。

最终得到的内核镜像大小如下，小于4MiB.
```shell
$ ls -l ./arch/x86/boot/bzImage 
-rw-r--r-- 1 hao hao 3856912 Mar 19 11:55 arch/x86/boot/bzImage
```

最终的`.config`文件和`bzImage`文件位于`lab1/`文件夹下.

### 交叉编译

安装必要的环境和QEMU
```shell
$ sudo apt install crossbuild-essential-arm64 qemu-system-arm
```

在`linux-6.2.7/`文件夹下使用命令
```shell
$ make clean
$ make menuconfig ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu-
$ make ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- -j $((`nproc`-1) 
```

即可在`linux-6.2.7/arch/arm64/boot/`文件夹下得到对应的Image镜像文件，没有修改配置时得到的镜像文件有38MiB，进行一定修改，得到的文件约为12MiB大小。对其进行测试需要用`aarch64-linux-gnu-gcc`对`init.c`进行编译，得到对应的启动盘，之后使用如下命令
```shell
$ qemu-system-aarch64  -kernel ./linux-6.2.7/arch/arm64/boot/Image -initrd ./initrd.cpio.gz    -nographic  -machine virt -cpu cortex-a53 -append console=ttyAMA0     
```
即可在QEMU中执行。

但也遇到了问题，运行得到输出
```shell
...
[    2.217306] Run /init as init process

```
后没有对应的输出"Hello..."，不知道是什么原因。但可以看出是可以正常运行的。交叉编译镜像存储于`lab1/arm64/`文件夹下。

## 创建初始内存盘

按要求，依次执行，得到`initrd.cpio.gz`文件，执行以下指令
```shell
$ qemu-system-x86_64 -kernel ./bzImage       -nographic -append console=ttyS0 -initrd ./initrd.cpio.gz    
```
得到输出
```shell
...
[    2.141622] Run /init as init process
Hello! PB21111691
[    2.553201] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input2

```
此后停住，可知正常运行。之后将启动盘存储于`lab1/`文件夹下。

### 修改`init.c`使kernel panic

方法：去除`while(1) {}`后再使用QEMU测试时，即会提示kernel panic.
```shell
...
[    4.022557] Run /init as init process
Hello! PB211111691
[    4.295516] Kernel panic - not syncing: Attempted to kill init! exitcode=0x00000000
[    4.295516] CPU: 0 PID: 1 Comm: init Not tainted 6.2.7 #2
[    4.295516] Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.15.0-1 04/01/2014
[    4.295516] Call Trace:
[    4.295516]  <TASK>
[    4.295516]  dump_stack+0x28/0x40
[    4.295516]  panic+0x2cb/0x2f0
[    4.295516]  do_exit+0x82f/0xa20
[    4.295516]  do_group_exit+0x2b/0x80
[    4.295516]  __x64_sys_exit_group+0x17/0x20
[    4.295516]  do_syscall_64+0x41/0x90
[    4.295516]  entry_SYSCALL_64_after_hwframe+0x46/0xb0
[    4.295516] RIP: 0033:0x4468d1
[    4.295516] Code: b8 ff ff ff be e7 00 00 00 ba 3c 00 00 00 eb 16 66 0f 1f 84 00 00 00 00 00 89 d0 0f 05 48 3d 00 f0 ff ff 77 1c f4 89 f0 0f 05 <48> 3d 00 f0 ff ff 760
[    4.295516] RSP: 002b:00007ffc1b235e78 EFLAGS: 00000246 ORIG_RAX: 00000000000000e7
[    4.295516] RAX: ffffffffffffffda RBX: 00000000004c7290 RCX: 00000000004468d1
[    4.295516] RDX: 000000000000003c RSI: 00000000000000e7 RDI: 0000000000000000
[    4.295516] RBP: 0000000000000000 R08: ffffffffffffffb8 R09: 0000000000845750
[    4.295516] R10: 000000000000006f R11: 0000000000000246 R12: 00000000004c7290
[    4.295516] R13: 0000000000000000 R14: 00000000004c7d60 R15: 0000000000401b20
[    4.295516]  </TASK>
[    4.295516] Kernel Offset: disabled
[    4.295516] ---[ end Kernel panic - not syncing: Attempted to kill init! exitcode=0x00000000 ]---
```
根据报错判断，输出是正确的，但是之后发生了kernel panic，因此应当是在结束时出现错误，可能是执行`return`后，`PC`处于错误的地址，如特殊模式下才可以执行的命令上。

## 添加自定义系统调用

按要求，编译新的内核，存储于`lab1/syscall/`文件夹下。

之后对`sys_hello`函数进行测试，在`temp/`文件夹下编写测试文件`initrd.c`，其中使用`syscall`函数进行系统调用，因为之前定义`sys_hello`函数对应的值为548，则定义宏`SYS_hello`值为548，使用
```c
syscall(SYS_hello, buf, 40);
syscall(SYS_hello, buf, 0);
```
则分别对应`buffer`长度充足和不充足时的结果，对结果进行分类判断，具体内容见`lab1/syscall/initrd.c`.

注意生成内存盘时在`temp`文件夹下的命令为
```shell
$ gcc -static initrd.c -o init
$ find . | cpio --quiet -H newc -o | gzip -9 -n > ../initrd.cpio.gz
```
之后再使用QEMU在该启动盘上运行添加系统调用的镜像，得到结果如下
```shell
...
[    2.459758] Run /init as init process
test enough buffer: enough buffer, 1815cec5210c289ac74b1aadbd167c7b

test not enough buffer: not enough buffer
[    2.668726] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input2

```
符合要求。

## lab1文件结构

```
- lab1
  - arm64
    - bzImage
  - syscall
    - bzImage
    - initrd.c
  - .config
  - bzImage
  - initrd.cpio.gz
  - README.md
```