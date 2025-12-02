#include <fstream>
#include <iostream>
#include <string>

#define INPUT_PATH "src/00/input.txt"

int
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    while (std::getline(input, line))
    {
        std::cout << line << std::endl;
    }

    input.close();

    return 0;
}

int
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    while (std::getline(input, line))
    {
        std::cout << line << std::endl;
    }

    input.close();

    return 0;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
