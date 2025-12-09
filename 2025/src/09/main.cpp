#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#define INPUT_PATH "src/09/input.txt"

struct rect {
    int64_t x_min, x_max, y_min, y_max;

    inline bool collides(const rect &other)
    {
        return x_min < other.x_max &&
               x_max > other.x_min &&
               y_min < other.y_max &&
               y_max > other.y_min;
    }
};

int64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<std::pair<int64_t, int64_t>> tiles;
    std::vector<int64_t> areas;

    while (std::getline(input, line))
    {
        int64_t x, y;
        sscanf(line.c_str(), "%ld,%ld", &x, &y);

        for (size_t i = 0; i < tiles.size(); ++i)
            areas.emplace_back((1 + std::abs(tiles[i].first - x)) * (1 + std::abs(tiles[i].second - y)));

        tiles.emplace_back(x, y);
    }

    input.close();

    std::sort(areas.begin(), areas.end(), [](const auto &a, const auto &b) {
        return a > b;
    });

    return areas.front();
}

int64_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<std::pair<int64_t, int64_t>> tiles;
    std::vector<std::tuple<int64_t, size_t, size_t>> areas;

    while (std::getline(input, line))
    {
        int64_t x, y;
        sscanf(line.c_str(), "%ld,%ld", &x, &y);

        for (size_t i = 0; i < tiles.size(); ++i)
        {
            areas.emplace_back(
                (1 + std::abs(tiles[i].first - x)) * (1 + std::abs(tiles[i].second - y)),
                i,
                tiles.size());
        }

        tiles.emplace_back(x, y);
    }

    input.close();

    std::sort(areas.begin(), areas.end(), [](const auto &a, const auto &b) {
        return std::get<0>(a) > std::get<0>(b);
    });

    for (const auto &[size, i, j] : areas)
    {
        const auto &[xi, yi] = tiles[i];
        const auto &[xj, yj] = tiles[j];

        rect current_rect = {
            .x_min = std::min(xi, xj),
            .x_max = std::max(xi, xj),
            .y_min = std::min(yi, yj),
            .y_max = std::max(yi, yj),
        };

        bool valid = true;
        size_t prev_tile = i;

        do
        {
            size_t curr_tile = (prev_tile + 1) % tiles.size();

            const auto &[x_prev, y_prev] = tiles[prev_tile];
            const auto &[x_curr, y_curr] = tiles[curr_tile];

            rect current_edge = {
                .x_min = std::min(x_curr, x_prev),
                .x_max = std::max(x_curr, x_prev),
                .y_min = std::min(y_curr, y_prev),
                .y_max = std::max(y_curr, y_prev),
            };

            if (current_rect.collides(current_edge))
            {
                valid = false;
                break;
            }

            prev_tile = curr_tile;
        } while (prev_tile != i);

        if (valid)
            return size;
    }

    return 0;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
