#include <fstream>
#include <iostream>
#include <string>

#define INPUT_PATH "src/01/input.txt"

int
part_one()
{
    std::ifstream input;
    std::string line;

    unsigned int count = 0;
    int position = 50;

    input.open(INPUT_PATH);

    while (std::getline(input, line))
    {

        int direction = line[0] == 'L' ? -1 : 1;
        int distance = std::stoi(line.substr(1));

        position += direction * distance;
        position %= 100;
        if (position < 0)
            position += 100;

        if (position == 0)
            ++count;
    }

    input.close();

    return count;
}

int
part_two()
{
    std::ifstream input;
    std::string line;

    unsigned int count = 0;
    int position = 50;

    input.open(INPUT_PATH);

    while (std::getline(input, line))
    {
        int distance = std::stoi(line.substr(1));

        if (line[0] == 'L')
        {
            if (position == 0)
            {
                count += distance / 100;
            }
            else if (distance >= position)
            {
                count += 1 + (distance - position) / 100;
            }

            position = (position - distance) % 100;
            if (position < 0)
                position += 100;
        }
        else if (line[0] == 'R')
        {
            if (position == 0)
            {
                count += distance / 100;
            }
            else if (distance >= 100 - position)
            {
                count += 1 + (distance - (100 - position)) / 100;
            }

            position = (position + distance) % 100;
        }
    }

    input.close();

    return count;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
