#include <cmath>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#define INPUT_PATH "src/02/input.txt"

long
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    long result = 0;

    while (std::getline(input, line, ','))
    {
        size_t dash = line.find('-');

        unsigned long first_id = std::stol(line.substr(0, dash));
        unsigned long last_id = std::stol(line.substr(dash + 1));

        for (unsigned long id = first_id; id <= last_id; ++id)
        {
            /*
            int num_digits = static_cast<int>(std::log10(id)) + 1;

            // odd number of digits is always valid
            if (num_digits & 1)
                continue;

            unsigned long left = id / std::pow(10, num_digits >> 1);
            unsigned long right = id - left * std::pow(10, num_digits >> 1);

            if (left == right)
                result += id;
            */

            std::string str = std::to_string(id);

            // odd number of digits is always valid
            if (str.size() & 1)
                continue;

            std::string left = str.substr(0, str.size() >> 1);
            std::string right = str.substr(str.size() >> 1);

            if (left == right)
                result += id;
        }
    }

    input.close();

    return result;
}

long
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    long result = 0;

    while (std::getline(input, line, ','))
    {
        size_t dash = line.find('-');

        unsigned long first_id = std::stol(line.substr(0, dash));
        unsigned long last_id = std::stol(line.substr(dash + 1));

        for (unsigned long id = first_id; id <= last_id; ++id)
        {
            std::string str = std::to_string(id);

            for (int i = 1; i <= str.size() / 2; ++i)
            {
                // skip non-divisors
                if (str.size() % i)
                    continue;

                int repeats = str.size() / i;

                bool invalid = true;

                for (int j = 0; j < repeats - 1; ++j)
                    if (str.substr(j * i, i) != str.substr((j + 1) * i, i))
                        invalid = false;

                if (invalid)
                {
                    result += id;
                    break;
                }
            }
        }
    }

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
