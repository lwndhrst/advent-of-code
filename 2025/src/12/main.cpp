#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#define INPUT_PATH "src/12/input.txt"

std::vector<std::string>
split(const std::string &s, char d)
{
    std::vector<std::string> result;

    std::stringstream ss(s);
    std::string item;

    while (getline(ss, item, d))
        result.push_back(item);

    return result;
}

// Always 6 shapes with 3x3 size
struct shape {
    std::array<bool, 9> positions;

    shape(const std::vector<std::string> &lines)
    {
        for (size_t i = 0; i < 3; ++i)
            for (size_t j = 0; j < 3; ++j)
                positions[i * 3 + j] = lines[i][j] == '#';
    }

    // rotate clockwise 90 degrees
    void rotate()
    {
        std::array<bool, 9> old_positions = positions;

        positions[0] = old_positions[6];
        positions[1] = old_positions[3];
        positions[2] = old_positions[0];

        positions[3] = old_positions[7];
        positions[4] = old_positions[4];
        positions[5] = old_positions[1];

        positions[6] = old_positions[8];
        positions[7] = old_positions[5];
        positions[8] = old_positions[2];
    }

    void flip_h()
    {
        std::array<bool, 9> old_positions = positions;

        positions[0] = old_positions[2];
        positions[3] = old_positions[5];
        positions[6] = old_positions[8];

        positions[2] = old_positions[0];
        positions[5] = old_positions[3];
        positions[8] = old_positions[6];
    }

    void flip_v()
    {
        std::array<bool, 9> old_positions = positions;

        positions[0] = old_positions[6];
        positions[1] = old_positions[7];
        positions[2] = old_positions[8];

        positions[6] = old_positions[0];
        positions[7] = old_positions[1];
        positions[8] = old_positions[2];
    }
};

struct region {
    size_t width, height;
    std::array<size_t, 6> shapes;

    region(const std::string &line)
    {
        size_t colon = line.find(':');

        std::vector<std::string> dimensions_str = split(line.substr(0, colon), 'x');
        width = std::stoul(dimensions_str[0]);
        height = std::stoul(dimensions_str[1]);

        std::vector<std::string> shapes_str = split(line.substr(colon + 2), ' ');
        for (size_t i = 0; i < 6; ++i)
            shapes[i] = std::stoul(shapes_str[i]);
    }
};

int64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<shape> shapes;
    shapes.reserve(6);

    std::vector<std::string> current_shape_lines;
    current_shape_lines.reserve(3);

    std::vector<region> regions;

    while (std::getline(input, line))
    {
        // Shapes
        if (shapes.size() < 6)
        {
            if (line.find(':') != std::string::npos)
            {
                current_shape_lines.clear();
                continue;
            }

            if (line == "")
            {
                shapes.emplace_back(shape(current_shape_lines));
                continue;
            }

            current_shape_lines.push_back(line);
            continue;
        }

        // Regions
        regions.emplace_back(region(line));
    }

    input.close();

    return 0;
}

int64_t
part_two()
{
    return 0;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
