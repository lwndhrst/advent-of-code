#include <cstring>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int
main()
{
    std::ifstream input;
    std::string line;

    input.open("src/01/input");

    int sum = 0;

    /* part one
    while (std::getline(input, line)) {
        std::string num;

        std::cout << line << "\n";

        for (int i = 0;; ++i) {
            if (line[i] >= '1' && line[i] <= '9') {
                num.push_back(line[i]);
                std::cout << "first at " << i << ": " << line[i] << "\n";
                break;
            }
        }

        for (int i = line.length() - 1;; --i) {
            if (line[i] >= '1' && line[i] <= '9') {
                num.push_back(line[i]);
                std::cout << "last at " << i << ": " << line[i] << "\n";
                break;
            }
        }

        std::cout << "num: " << num << "\n\n";

        sum += std::stoi(num);
    }
    */

    /* part two */
    std::unordered_map<std::string, int> map = {};
    map["one"] = '1';
    map["two"] = '2';
    map["three"] = '3';
    map["four"] = '4';
    map["five"] = '5';
    map["six"] = '6';
    map["seven"] = '7';
    map["eight"] = '8';
    map["nine"] = '9';

    while (std::getline(input, line)) {
        std::string num;

        char first_val = '0';
        char last_val = '0';
        int first_pos = line.length();
        int last_pos = 0;
        for (const auto &[key, val] : map) {
            int pos = line.find(key);
            if (pos == -1) {
                continue;
            }

            if (pos < first_pos) {
                first_pos = pos;
                first_val = val;
            }

            for (int i = line.length(); i > 0; --i) {
                int pos = line.find(key, i);

                if (pos == -1) {
                    continue;
                }

                if (pos > last_pos && pos < line.length()) {
                    last_pos = pos;
                    last_val = val;
                }
            }
        }

        for (int i = 0;; ++i) {
            if (line[i] >= '1' && line[i] <= '9') {
                if (i < first_pos) {
                    first_pos = i;
                    first_val = line[i];
                }
                break;
            }
        }

        for (int i = line.length();; --i) {
            if (line[i] >= '1' && line[i] <= '9') {
                if (i > last_pos) {
                    last_pos = i;
                    last_val = line[i];
                }
                break;
            }
        }

        num.push_back(first_val);
        num.push_back(last_val);

        std::cout << line << "\n";
        std::cout << "first at " << first_pos << ": " << first_val << "\n";
        std::cout << "last at " << last_pos << ": " << last_val << "\n";
        std::cout << "num: " << num << "\n\n";

        sum += std::stoi(num);
    }

    std::cout << sum << std::endl;

    return EXIT_SUCCESS;
}
