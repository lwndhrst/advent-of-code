#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#define INPUT_PATH "src/08/input.txt"

typedef std::pair<size_t, size_t> connection;

struct vec3 {
    int64_t x, y, z;

    vec3(const std::string &line)
    {
        size_t first_comma = line.find_first_of(',');
        size_t last_comma = line.find_last_of(',');

        x = std::stoull(line.substr(0));
        y = std::stoull(line.substr(first_comma + 1));
        z = std::stoull(line.substr(last_comma + 1));
    }

    float sqr_dist(const vec3 &other)
    {
        float dx = x - other.x;
        float dy = y - other.y;
        float dz = z - other.z;

        return dx * dx + dy * dy + dz * dz;
    }
};

struct union_find {
    std::vector<size_t> parents;
    std::vector<size_t> set_sizes;
    size_t set_count;

    union_find(size_t size)
    {
        parents.resize(size);
        set_sizes.resize(size);
        set_count = size;

        for (size_t i = 0; i < size; ++i)
        {
            parents[i] = i;
            set_sizes[i] = 1;
        }
    }

    size_t representative(size_t i)
    {
        return parents[i] == i ? i : representative(parents[i]);
    }

    void merge(size_t i, size_t j)
    {
        size_t ri = representative(i);
        size_t rj = representative(j);

        if (ri == rj)
            return;

        size_t tmp = set_sizes[rj];
        set_sizes[rj] = 0;
        set_sizes[ri] += tmp;

        parents[rj] = ri;

        --set_count;
    }
};

int64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<vec3> positions;

    while (std::getline(input, line))
        positions.emplace_back(vec3(line));

    input.close();

    std::vector<std::pair<float, connection>> connections;

    for (size_t i = 0; i < positions.size(); ++i)
        for (size_t j = i + 1; j < positions.size(); ++j)
            connections.push_back({positions[i].sqr_dist(positions[j]), {i, j}});

    union_find uf(positions.size());

    std::sort(connections.begin(), connections.end());

    for (size_t i = 0; i < 1000; ++i)
    {
        const auto &[from, to] = connections[i].second;
        uf.merge(from, to);
    }

    std::sort(uf.set_sizes.begin(), uf.set_sizes.end(), [](size_t a, size_t b) {
        return a > b;
    });

    int64_t result = 1;

    for (size_t i = 0; i < 3; ++i)
        result *= uf.set_sizes[i];

    return result;
}

int64_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<vec3> positions;

    while (std::getline(input, line))
        positions.emplace_back(vec3(line));

    input.close();

    std::vector<std::pair<float, connection>> connections;

    for (size_t i = 0; i < positions.size(); ++i)
        for (size_t j = i + 1; j < positions.size(); ++j)
            connections.push_back({positions[i].sqr_dist(positions[j]), {i, j}});

    union_find uf(positions.size());

    std::sort(connections.begin(), connections.end());

    int64_t result = 0;

    for (size_t i = 0; i < connections.size(); ++i)
    {
        const auto &[from, to] = connections[i].second;
        uf.merge(from, to);

        if (uf.set_count == 1)
        {
            result = positions[from].x * positions[to].x;
            break;
        }
    }

    return result;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
