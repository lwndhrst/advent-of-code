#include <cctype>
#include <fstream>
#include <functional>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int
main()
{
    std::ifstream input;
    std::string line;

    input.open("src/03/input");

    std::vector<std::string> arr;
    arr.reserve(142);

    int line_len;
    while (std::getline(input, line)) {
        if (arr.empty()) {
            line_len = line.length() + 2;
            arr.push_back(std::string(line_len, '.'));
        }
        arr.push_back('.' + line + '.');
    }
    arr.push_back(std::string(line_len, '.'));

    int part_1 = 0;
    int part_2 = 0;

    std::vector<std::function<char(int, int)>> directions;

    auto up = [arr](int i, int j) { return arr[i - 1][j]; };
    auto down = [arr](int i, int j) { return arr[i + 1][j]; };
    auto left = [arr](int i, int j) { return arr[i][j - 1]; };
    auto right = [arr](int i, int j) { return arr[i][j + 1]; };
    directions.insert(directions.end(), {up, down, left, right});

    auto up_left = [arr](int i, int j) { return arr[i - 1][j - 1]; };
    auto up_right = [arr](int i, int j) { return arr[i - 1][j + 1]; };
    auto down_left = [arr](int i, int j) { return arr[i + 1][j - 1]; };
    auto down_right = [arr](int i, int j) { return arr[i + 1][j + 1]; };
    directions.insert(directions.end(), {up_left, up_right, down_left, down_right});

    /* part one */
    for (int i = 1; i < arr.size() - 1; ++i) {
        int num = 0;
        int num_valid = false;
        for (int j = 1; j < line_len; ++j) {
            if (std::isdigit(arr[i][j])) {
                num = 10 * num + arr[i][j] - 48;

                for (auto &dir : directions) {
                    if (!std::isdigit(dir(i, j)) && dir(i, j) != '.') {
                        num_valid = true;
                        break;
                    }
                }
            } else {
                if (num_valid) {
                    part_1 += num;
                }
                num = 0;
                num_valid = false;
            }
        }
    }

    /* part two */
    auto parse_left_num = [arr, left, right](int row, int col) {
        int num_val = 0;
        while (std::isdigit(left(row, col))) {
            col -= 1;
        }
        while (std::isdigit(arr[row][col])) {
            num_val = 10 * num_val + arr[row][col] - 48;
            col += 1;
        }
        return num_val;
    };

    auto parse_right_num = [arr, right](int row, int col) {
        int num_val = 0;
        while (std::isdigit(arr[row][col])) {
            num_val = 10 * num_val + arr[row][col] - 48;
            col += 1;
        }
        return num_val;
    };

    for (int i = 1; i < arr.size() - 1; ++i) {
        for (int j = 1; j < line_len; ++j) {
            if (arr[i][j] == '*') {
                int adj_nums = 0;
                int gear_ratio = 1;

                if (std::isdigit(left(i, j))) {
                    adj_nums += 1;
                    gear_ratio *= parse_left_num(i, j - 1);
                }

                if (std::isdigit(right(i, j))) {
                    adj_nums += 1;
                    gear_ratio *= parse_right_num(i, j + 1);
                }

                if (std::isdigit(up(i, j))) {
                    adj_nums += 1;
                    gear_ratio *= parse_left_num(i - 1, j);
                } else {
                    if (std::isdigit(up_left(i, j))) {
                        adj_nums += 1;
                        gear_ratio *= parse_left_num(i - 1, j - 1);
                    }
                    if (std::isdigit(up_right(i, j))) {
                        adj_nums += 1;
                        gear_ratio *= parse_right_num(i - 1, j + 1);
                    }
                }

                if (std::isdigit(down(i, j))) {
                    adj_nums += 1;
                    gear_ratio *= parse_left_num(i + 1, j);
                } else {
                    if (std::isdigit(down_left(i, j))) {
                        adj_nums += 1;
                        gear_ratio *= parse_left_num(i + 1, j - 1);
                    }
                    if (std::isdigit(down_right(i, j))) {
                        adj_nums += 1;
                        gear_ratio *= parse_right_num(i + 1, j + 1);
                    }
                }

                if (adj_nums != 2) {
                    continue;
                }

                part_2 += gear_ratio;
            }
        }
    }

    std::cout << "part one: " << part_1 << "\n";
    std::cout << "part two: " << part_2 << "\n";

    return EXIT_SUCCESS;
}
