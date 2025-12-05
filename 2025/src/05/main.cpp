#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#define INPUT_PATH "src/05/input.txt"

uint64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    bool before_blank = true;
    std::vector<std::pair<uint64_t, uint64_t>> ranges;

    uint64_t result = 0;

    while (std::getline(input, line))
    {
        if (line.size() == 0)
        {
            before_blank = false;
            continue;
        }

        if (before_blank)
        {
            size_t dash = line.find('-');
            uint64_t lb = std::stoul(line.substr(0, dash));
            uint64_t ub = std::stoul(line.substr(dash + 1));

            ranges.emplace_back(lb, ub);
        }
        else
        {
            uint64_t id = std::stoul(line);

            for (const auto &[lb, ub] : ranges)
            {
                if (id >= lb && id <= ub)
                {
                    ++result;
                    break;
                }
            }
        }
    }

    input.close();

    return result;
}

uint64_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<std::pair<uint64_t, uint64_t>> ranges;

    uint64_t result = 0;

    while (std::getline(input, line))
    {
        if (line.size() == 0)
            break;

        size_t dash = line.find('-');
        uint64_t lb = std::stoul(line.substr(0, dash));
        uint64_t ub = std::stoul(line.substr(dash + 1));

        ranges.emplace_back(lb, ub);
    }

    std::sort(ranges.begin(), ranges.end());

    // merge ranges
    std::vector<std::pair<uint64_t, uint64_t>> merged_ranges;
    std::pair<uint64_t, uint64_t> current_range = ranges[0];
    for (const auto &[lb, ub] : ranges)
    {
        if (lb <= current_range.second)
            current_range.second = std::max(current_range.second, ub);
        else
        {
            merged_ranges.push_back(current_range);
            current_range = {lb, ub};
        }
    }
    merged_ranges.push_back(current_range);

    for (const auto &[lb, ub] : merged_ranges)
        result += (ub - lb) + 1;

    input.close();

    return result;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
