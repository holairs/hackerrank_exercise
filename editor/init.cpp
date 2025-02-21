#include <iostream>
#include <string>
#include <vector>

int main() {
    int q;
    std::cin >> q;
    std::string text = "";
    std::vector<std::string> history = {""};
    for (int i = 0; i < q; ++i) {
        int operation_type;
        std::cin >> operation_type;
        if (operation_type == 1) {
            std::string append_str;
            std::cin >> append_str;
            history.push_back(text);
            text += append_str;
        } else if (operation_type == 2) {
            int delete_count;
            std::cin >> delete_count;
            history.push_back(text);
            text.erase(text.length() - delete_count);
        } else if (operation_type == 3) {
            int print_index;
            std::cin >> print_index;
            std::cout << text[print_index - 1] << std::endl;
        } else if (operation_type == 4) {
            history.pop_back();
            text = history.back();
        }
    }
    return 0;
}
