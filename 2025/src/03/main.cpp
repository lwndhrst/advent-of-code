#include <fstream>
#include <iostream>
#include <string>

#define INPUT_PATH "src/03/input.txt"

long
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    long result = 0;

    while (std::getline(input, line))
    {
        char first = line[0];
        char second = line[1];

        for (int i = 2; i < line.size(); ++i)
        {
            if (first < second)
            {
                first = second;
                second = line[i];
            }
            else if (second < line[i])
            {
                second = line[i];
            }
        }

        result += (first - 48) * 10 + (second - 48);
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

    while (std::getline(input, line))
    {
        std::string activated = line.substr(0, 12);

        for (int i = 12; i < line.size(); ++i)
        {
            bool changed = false;

            for (int j = 0; j < 11; ++j)
            {
                if (activated[j] < activated[j + 1])
                {
                    activated.erase(j, 1);
                    activated.push_back(line[i]);
                    changed = true;
                    break;
                }
            }

            if (!changed && activated[11] < line[i])
            {
                activated[11] = line[i];
            }
        }

        result += std::stol(activated);
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
