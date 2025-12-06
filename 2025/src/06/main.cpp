#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#define INPUT_PATH "src/06/input.txt"

int64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<int64_t> nums;
    std::vector<char> ops;

    while (std::getline(input, line))
    {
        for (size_t i = 0; i < line.size(); ++i)
        {
            if (line[i] == ' ')
                continue;

            size_t j = i + 1;
            while (j < line.size() && line[j] != ' ')
                ++j;

            std::string token = line.substr(i, j - i);

            if (token.front() == '+' || token.front() == '*')
                ops.emplace_back(token[0]);
            else
                nums.emplace_back(std::stol(token));

            i = j;
        }
    }

    int64_t result = 0;

    for (size_t i = 0; i < ops.size(); ++i)
    {
        char op = ops[i];

        if (op == '+')
        {
            int64_t sum = 0;

            for (size_t j = i; j < nums.size(); j += ops.size())
                sum += nums[j];

            result += sum;
        }
        else
        {
            int64_t product = 1;

            for (size_t j = i; j < nums.size(); j += ops.size())
                product *= nums[j];

            result += product;
        }
    }

    input.close();

    return result;
}

int64_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<std::string> worksheet;

    while (std::getline(input, line))
        worksheet.push_back(line);

    input.close();

    int64_t result = 0;

    std::string &ops = worksheet.back();

    std::vector<int64_t> nums;
    char op = ops.front();

    const auto &compute = [&]() {
        if (op == '+')
        {
            int64_t sum = 0;

            for (size_t j = 0; j < nums.size(); ++j)
                sum += nums[j];

            result += sum;
        }
        else
        {
            int64_t product = 1;

            for (size_t j = 0; j < nums.size(); ++j)
                product *= nums[j];

            result += product;
        }
    };

    for (size_t i = 0; i < ops.size(); ++i)
    {
        if (i + 1 < ops.size() && ops[i + 1] != ' ')
        {
            compute();

            nums.clear();
            op = ops[i + 1];
            continue;
        }

        std::string num = "";
        for (size_t j = 0; j < worksheet.size() - 1; ++j)
        {
            const char &c = worksheet[j][i];
            if (c == ' ')
                continue;

            num.push_back(c);
        }

        if (num.size() > 0)
            nums.emplace_back(std::stol(num));
    }

    compute();

    return result;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
