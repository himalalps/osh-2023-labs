# 实验三：简易 HTTP/1.0 服务器

## 编译说明

在以下说明的对应目录下直接使用`cargo build --release`即可在对应目录的`target/release/`文件夹下找到可执行文件`server`。

### 多线程版本

位于`./server_thread/`目录下。

代码见`src/`目录，`main.rs`为入口，网络请求使用`TcpListener::bind()`绑定对应的监听IP和端口，对`listner.incoming()`函数返回的迭代器进行遍历得到`TcpStream`对象，并发处理使用`thread::spawn()`函数，其中对该对象调用了`lib.rs`的`handle_connection`函数。之后依次根据`parse_request`的结果将不同的返回内容写入`TcpStream`中。

### 线程池版本

位于`./server_thread_pool/`目录下。

相比多线程版本，加入了`pool.rs`内的`ThreadPool`相关内容，首先在`main.rs`中创建32个线程的线程池`pool`，之后对于多线程中的`thread::spawn()`都使用`pool.execute()`替代，这样就将任务提交给了线程池。为了方便线程池停机，使用`signal_hook`接收`SIGINT`信号则跳出循环，编译器会自动调用`ThreadPool`的`drop`函数。

### 异步版本

位于`./server_async/`目录下。

