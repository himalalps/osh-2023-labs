# 实验三：简易 HTTP/1.0 服务器

## 编译说明

在以下说明的对应目录下直接使用`cargo build --release`即可在对应目录的`target/release/`文件夹下找到可执行文件`server`。

### 多线程版本

位于`./server_thread/`目录下。

代码见`src/`目录，`main.rs`为入口，网络请求使用`TcpListener::bind()`绑定对应的监听IP和端口，对`listner.incoming()`函数返回的迭代器进行遍历得到`TcpStream`对象，并发处理使用`thread::spawn()`函数，其中对该对象调用了`lib.rs`的`handle_connection`函数。

该函数使用`parse_request`处理得到的`TcpStream`，使用正则表达式解析请求头，接着判断是否为目录，是否可正确读取，分别得到`Ok`, `NotFound`和`ServerError`的结果以及可能读取到的文件内容，再将这些结果按照`HTTP`返回头要求的格式转化为`&[u8]`类型写入`TcpStream`中，这样就处理了一次请求。

### 线程池版本

位于`./server_thread_pool/`目录下。

在`main.rs`中创建32个线程的线程池`pool`，之后对于多线程中的`thread::spawn()`都使用`pool.execute()`替代，这样就将任务提交给了线程池。为了方便线程池停机，使用`signal_hook`接收`SIGINT`信号则跳出循环，编译器会自动调用`ThreadPool`的`drop`函数。而处理请求的内容与多线程版本一样。

相比多线程版本，加入了`pool.rs`内的`ThreadPool`相关内容。新建线程池的时候就初始化一系列的`Worker`存在`Vec`中，每个`Worker`新建的时候会传入一个`receiver`，用于之后接收任务。而`sender`存在线程池里面，这样调用`pool.execute()`时，由线程池发送任务，一个`worker`接收到之后就运行对应的`job`。`receiver`和`sender`在新建线程池的时候就使用`mpsc::channel()`创建。同时，因为`mpsc`对应多个生产者单个消费者，为了每个`worker`都能接收信号，需要`Arc`来共享以及`Mutex`来保证线程间的接收信号的互斥。

当`ThreadPool`离开作用域，`drop`函数就会自动执行，首先对`sender`调用`drop`函数，这样就关闭了用于发送任务的通道，而`mpsc::channel()`会在所有发送端关闭后自动关闭，所以只需要在这时候对每个`worker`进行遍历，对其线程调用`thread.join()`等待线程结束，就可以保证所有线程都结束保证停机。

> 本版本参考了[The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html)中的线程池实现。

### 异步版本

位于`./server_async/`目录下。

使用了`tokio`异步库，将原本的`std`下的函数除去`path`外都转化为了`tokio`内的函数，即转化为异步的版本。而异步情况下，函数返回的是`Future`，所以需要`.await`保证其可以执行。

`TcpListener`需要转为非阻塞的`accept()`而非之前的`incoming()`。`handle_connection`也变为异步版本。其中涉及IO的部分都加入了`.await`这些也是可能会较耗时的部分。

