#include <fstream>
#include <iostream>
#include <string>

int
main()
{
    std::ifstream input;
    std::string line;

    input.open("src/01/input.txt");

    unsigned int count = 0;
    int position = 50;

    while (std::getline(input, line))
    {
        int direction = line[0] == 'L' ? -1 : 1;
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
        }

        position += direction * distance;
        position %= 100;
        if (position < 0)
            position += 100;
    }

    std::cout << count << std::endl;

    return 0;
}
