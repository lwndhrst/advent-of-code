#include <fstream>
#include <iostream>
#include <string>

int
main()
{
    std::ifstream input;
    std::string line;

    input.open("src/01/input");
    while (std::getline(input, line)) {
        std::cout << line << std::endl;
    }

    return EXIT_SUCCESS;
}
