// IO
#include <iostream>
// std::string
#include <string>
// std::vector
#include <vector>
// std::string 转 int
#include <sstream>
// PATH_MAX 等常量
#include <climits>
// POSIX API
#include <unistd.h>
// wait
#include <sys/wait.h>
// 用于获取用户
#include <pwd.h>

#define READ_END 0
#define WRITE_END 1

std::vector<std::string> split(std::string s, const std::string &delimiter);
void trim(std::string &line);
void format(std::string &line);
void extend(std::vector<std::string> &args, char *home);
void pwd(const std::vector<std::string> &args);
void cd(std::vector<std::string> &args, const char *home);
void exec(std::string &cmd, char *home);

int main() {
    // 不同步 iostream 和 cstdio 的 buffer
    std::ios::sync_with_stdio(false);

    // 用来存储读入的一行命令
    std::string cmd;

    // 用于替换路径中的'~'
    char *home = getenv("HOME");
    if (home == nullptr) {
        home = getpwuid(getuid())->pw_dir;
    }

    while (true) {
        // 打印提示符
        std::cout << "# ";

        // 读入一行。std::getline 结果不包含换行符。
        std::getline(std::cin, cmd);

        // 命令格式化
        format(cmd);

        // 按空格分割命令为单词
        std::vector<std::string> args = split(cmd, " ");

        // 没有可处理的命令
        if (args.empty()) {
            continue;
        }

        // 退出
        if (args[0] == "exit") {
            if (args.size() <= 1) {
                return 0;
            }

            // std::string 转 int
            std::stringstream code_stream(args[1]);
            int code = 0;
            code_stream >> code;

            // 转换失败
            if (!code_stream.eof() || code_stream.fail()) {
                std::cout << "Invalid exit code\n";
                continue;
            }

            return code;
        }

        // 处理pwd命令
        if (args[0] == "pwd") {
            pwd(args);
            continue;
        }

        extend(args, home);

        // 处理cd命令
        if (args[0] == "cd") {
            cd(args, home);
            continue;
        }

        // // std::vector<std::string> 转 char **
        // char *arg_ptrs[args.size() + 1];

        // 根据管道分割命令
        std::vector<std::string> cmds = split(cmd, "|");
        // 管道数量
        int pipe_num = cmds.size() - 1;

        // 预处理
        // preprocess(poses, args, arg_ptrs, pipe_num, home);

        // 处理外部命令
        if (pipe_num > 0) {
            // 存在管道
            int pipefd[pipe_num][2];
            for (int i = 0; i < pipe_num; i++) {
                if (pipe(pipefd[i]) == -1) {
                    std::cout << "pipe failed" << std::endl;
                    exit(255);
                }
            }
            for (int i = 0; i < pipe_num + 1; i++) {
                pid_t cpid = fork();
                if (cpid < 0) {
                    std::cout << "fork failed" << std::endl;
                    exit(255);
                } else if (cpid == 0) {
                    // 输出重定向
                    if (i == 0) { // 第一个子进程输入来自标准输入
                        dup2(pipefd[i][WRITE_END], STDOUT_FILENO);
                    } else if (i == pipe_num) { // 最后一个子进程输出到标准输出
                        dup2(pipefd[i - 1][READ_END], STDIN_FILENO);
                    } else { // 其余进程输入输出都是管道
                        dup2(pipefd[i - 1][READ_END], STDIN_FILENO);
                        dup2(pipefd[i][WRITE_END], STDOUT_FILENO);
                    }

                    exec(cmds[i], home);
                }
                if (i == 0) { // 第一个进程关闭原本的标准输出
                    close(pipefd[i][WRITE_END]);
                } else if (i == pipe_num) { // 最后一共进程关闭原本的标准输入
                    close(pipefd[i - 1][READ_END]);
                } else { // 其余进程关闭原本的标准输入和输出
                    close(pipefd[i - 1][READ_END]);
                    close(pipefd[i][WRITE_END]);
                }
            }
            // 等待所有子进程结束
            while (wait(nullptr) > 0)
                ;
        } else {
            // 不存在管道
            pid_t pid = fork();

            if (pid < 0) {
                std::cout << "fork failed" << std::endl;
            } else if (pid == 0) {
                // 这里只有子进程才会进入
                exec(cmds[0], home);
            }
            // 这里只有父进程（原进程）才会进入
            int ret = wait(nullptr);
            if (ret < 0) {
                std::cout << "wait failed" << std::endl;
                exit(255);
            }
        }
    }
}

// 经典的 cpp string split 实现
// https://stackoverflow.com/a/14266139/11691878
std::vector<std::string> split(std::string s, const std::string &delimiter) {
    std::vector<std::string> res;
    size_t pos = 0;
    std::string token;
    while ((pos = s.find(delimiter)) != std::string::npos) {
        token = s.substr(0, pos);
        res.push_back(token);
        s = s.substr(pos + delimiter.length());
    }
    res.push_back(s);
    return res;
}

// 去除line前后空格
void trim(std::string &line) {
    int lPos = line.find_first_not_of(' ');
    if (lPos == -1) { // 整行均为' '
        line.replace(0, line.length(), "");
        return;
    }
    int rPos = line.find_last_not_of(' ');
    line.replace(0, line.length(), line.substr(lPos, rPos - lPos + 1));
    return;
}

// 命令行格式化
void format(std::string &line) {
    std::string replace_list = {'\t', '\n', '\r', '\f', '\v'};
    std::string::size_type pos;
    if (line.find('#') != std::string::npos) {
        line = line.substr(0, line.find('#'));
    }
    for (auto i : replace_list) {
        while ((pos = line.find(i)) != std::string::npos) {
            line.replace(pos, 1, " ");
        }
    }
    while ((pos = line.find("  ")) != std::string::npos) {
        line.replace(pos, 2, " ");
    }
    trim(line);
    return;
}

void extend(std::vector<std::string> &args, char *home) {
    for (std::size_t i = 0; i < args.size(); i++) {
        if (args[i].substr(0, 1) == "$") { // 替换环境变量
            size_t pos = args[i].find_first_of('/');
            if (pos == std::string::npos) {
                char *param = getenv(&(args[i].substr(1, args[i].length() - 1)[0]));
                if (param == nullptr) {
                    args[i].replace(0, args[i].length(), " ");
                } else {
                    args[i].replace(0, args[i].length(), param);
                }
            } else {
                char *param = getenv(getenv(&(args[i].substr(1, pos - 1)[0])));
                if (param == nullptr) {
                    args[i].replace(0, pos, " ");
                } else {
                    args[i].replace(0, pos, param);
                }
            }
        } else if (args[i].substr(0, 2) == "~/" || args[i] == "~") { // 替换 ~
            args[i].replace(0, 1, std::string(home));
        } else if (args[i].substr(0, 1) == "~") { // ~拓展
            size_t pos = args[i].find_first_of('/');
            if (pos == std::string::npos) {
                args[i].replace(0, args[i].length(), getpwnam(&(args[i].substr(1, args[i].length() - 1))[0])->pw_dir);
            } else {
                args[i].replace(0, pos, getpwnam(&(args[i].substr(1, pos - 1))[0])->pw_dir);
            }
        }
    }
    return;
}

// pwd 命令
void pwd(const std::vector<std::string> &args) {
    if (args.size() > 1) {
        std::cout << "pwd: invalid argument number" << std::endl;
        return;
    }
    char *buf = new char[PATH_MAX + 1];
    int size = PATH_MAX;
    while (getcwd(buf, size) == nullptr) {
        delete[] buf;
        size += PATH_MAX;
        buf = new char[size];
    }
    std::cout << buf << std::endl;
    delete[] buf;
    return;
}

// cd 命令
void cd(std::vector<std::string> &args, const char *home) {
    if (args.size() > 2) {
        std::cout << "cd: invalid argument number" << std::endl;
        return;
    }
    if (args.size() == 1) {
        args.push_back(std::string(home));
    } else if (args[1].substr(0, 2) == "~/" || args[1] == "~") {
        args[1].replace(0, 1, std::string(home));
    }
    if (chdir(args[1].c_str()) == -1) {
        std::cout << "cd failed" << std::endl;
    }
    return;
}

void exec(std::string &cmd, char *home) {
    format(cmd);

    if (cmd.find("<") != std::string::npos) {
        printf("input redirect\n");
    }

    if (cmd.find(">>") != std::string::npos) {
        printf("append redirect\n");
    } else if (cmd.find(">") != std::string::npos) {
        printf("output redirect\n");
    }

    std::vector<std::string> args = split(cmd, " ");
    extend(args, home);
    char *arg_ptrs[args.size() + 1];
    for (std::size_t i = 0; i < args.size(); i++) {
        arg_ptrs[i] = &args[i][0];
    }
    // exec p 系列的 argv 需要以 nullptr 结尾
    arg_ptrs[args.size()] = nullptr;

    // execvp 会完全更换子进程接下来的代码，所以正常情况下 execvp 之后这里的代码就没意义了
    // 如果 execvp 之后的代码被运行了，那就是 execvp 出问题了
    execvp(args[0].c_str(), arg_ptrs);

    // 所以这里直接报错
    exit(255);
}