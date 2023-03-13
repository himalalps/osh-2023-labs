#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> split(std::string s, const std::string &d) {
    std::vector<std::string> vec;
    std::size_t prev = 0;
    std::size_t cur = s.find_first_of(d);
    while (cur != std::string::npos) {
        if (cur > prev) {
            vec.push_back(s.substr(prev, cur - prev));
        }
        prev = cur + 1;
        cur = s.find_first_of(d, prev);
    }
    vec.push_back(s.substr(prev));
    return vec;
}

int main(void) {
    for (std::string s : split("1,2,3", ",")) {
        std::cout << s << std::endl;
    }
    return 0;
}