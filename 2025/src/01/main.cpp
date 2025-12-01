#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int
main()
{
    std::ifstream input;
    std::string line;

    input.open("src/01/input.txt");

    while (std::getline(input, line))
    {
        std::cout << line << std::endl;
    }

    return 0;
}
