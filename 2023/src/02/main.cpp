#include <cstring>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>

#define MAX_R 12
#define MAX_G 13
#define MAX_B 14

#define max(a, b) (a > b ? a : b)

int
main()
{
    std::ifstream input;
    std::string line;

    input.open("src/02/input");

    std::unordered_map<std::string, int> max_map;
    max_map["red"] = MAX_R;
    max_map["green"] = MAX_G;
    max_map["blue"] = MAX_B;

    int part_1 = 0;
    int part_2 = 0;

    while (std::getline(input, line)) {
        int first_space_pos = line.find(' ');
        int first_colon_pos = line.find(':');

        int game_id = std::stoi(line.substr(first_space_pos + 1, first_colon_pos - first_space_pos - 1));

        std::vector<std::string> rounds;
        int cur_pos = first_colon_pos + 1;
        while (cur_pos < line.length()) {
            int round_end_pos = line.find(';', cur_pos);
            if (round_end_pos == -1) {
                round_end_pos = line.length();
            }

            rounds.push_back(line.substr(cur_pos, round_end_pos - cur_pos));
            cur_pos = round_end_pos + 1;
        }

        std::unordered_map<std::string, int> occurrences;

        for (const auto &round : rounds) {
            int cur_pos = 0;
            while (cur_pos < round.length()) {
                int col_end_pos = round.find(',', cur_pos);
                if (col_end_pos == -1) {
                    col_end_pos = round.length();
                }

                std::string num_and_col = round.substr(cur_pos + 1, col_end_pos - cur_pos - 1);
                int split_pos = num_and_col.find(' ');

                std::string num = num_and_col.substr(0, split_pos);
                std::string col = num_and_col.substr(split_pos + 1, num_and_col.length() - split_pos - 1);

                if (occurrences.contains(col)) {
                    occurrences[col] = max(std::stoi(num), occurrences[col]);
                } else {
                    occurrences[col] = std::stoi(num);
                }

                cur_pos = col_end_pos + 1;
            }
        }

        bool possible = true;
        int power = 1;
        for (const auto &[col, num] : occurrences) {
            if (num > max_map[col]) {
                possible = false;
            }
            power *= num;
        }
        part_2 += power;

        // std::cout << line << "\n";
        // std::cout << "possible: " << possible << "\n";

        if (possible) {
            part_1 += game_id;
        }
    }

    std::cout << "part 1: " << part_1 << "\n";
    std::cout << "part 2: " << part_2 << "\n";

    return EXIT_SUCCESS;
}
