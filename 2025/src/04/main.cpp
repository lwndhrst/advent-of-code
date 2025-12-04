#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>

#define INPUT_PATH "src/04/input.txt"

long
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<std::string> grid;

    while (std::getline(input, line))
        grid.push_back(line);

    int rows = grid.size();
    int cols = grid[0].size();

    long result = 0;

    for (int row = 0; row < rows; ++row)
    {
        for (int col = 0; col < cols; ++col)
        {
            if (grid[row][col] != '@')
                continue;

            int u = std::max(row - 1, 0);
            int d = std::min(row + 1, rows - 1);

            int l = std::max(col - 1, 0);
            int r = std::min(col + 1, cols - 1);

            std::set<std::pair<int, int>> cells_to_check = {
                {u, l},
                {u, col},
                {u, r},
                {row, l},
                {row, col}, // count current cell and add 1 to max adjacencies
                {row, r},
                {d, l},
                {d, col},
                {d, r},
            };

            int adj = 0;

            for (const auto &[row, col] : cells_to_check)
                if (grid[row][col] == '@')
                    ++adj;

            if (adj < 5)
                ++result;
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

    std::vector<std::string> grid;

    while (std::getline(input, line))
        grid.push_back(line);

    int rows = grid.size();
    int cols = grid[0].size();

    long result = 0;

    for (;;)
    {
        bool changed = false;

        for (int row = 0; row < rows; ++row)
        {
            for (int col = 0; col < cols; ++col)
            {
                if (grid[row][col] != '@')
                    continue;

                int u = std::max(row - 1, 0);
                int d = std::min(row + 1, rows - 1);

                int l = std::max(col - 1, 0);
                int r = std::min(col + 1, cols - 1);

                std::set<std::pair<int, int>> cells_to_check = {
                    {u, l},
                    {u, col},
                    {u, r},
                    {row, l},
                    {row, col}, // count current cell and add 1 to max adjacencies
                    {row, r},
                    {d, l},
                    {d, col},
                    {d, r},
                };

                int adj = 0;

                for (const auto &[row, col] : cells_to_check)
                    if (grid[row][col] == '@')
                        ++adj;

                if (adj < 5)
                {
                    ++result;
                    grid[row][col] = '.';
                    changed = true;
                }
            }
        }

        if (!changed)
            break;
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
