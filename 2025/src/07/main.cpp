#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>

#define INPUT_PATH "src/07/input.txt"

int64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::unordered_set<size_t> beams;

    std::getline(input, line);

    size_t source = 0;
    while (line[source] != 'S')
        ++source;

    beams.insert(source);

    int64_t splits = 0;

    while (std::getline(input, line))
    {
        std::unordered_set<size_t> updated_beams;

        for (const size_t &beam : beams)
        {
            if (line[beam] == '^')
            {
                updated_beams.insert(beam - 1);
                updated_beams.insert(beam + 1);
                ++splits;
            }
            else
            {
                updated_beams.insert(beam);
            }
        }

        beams = updated_beams;
    }

    input.close();

    return splits;
}

int64_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::unordered_map<size_t, size_t> timelines;

    std::getline(input, line);

    size_t source = 0;
    while (line[source] != 'S')
        ++source;

    timelines[source] = 1;

    while (std::getline(input, line))
    {
        std::unordered_map<size_t, size_t> updated_timelines;

        for (const auto &[beam, count] : timelines)
        {
            if (line[beam] == '^')
            {
                updated_timelines[beam - 1] += timelines[beam];
                updated_timelines[beam + 1] += timelines[beam];
            }
            else
            {
                updated_timelines[beam] += timelines[beam];
            }
        }

        timelines = updated_timelines;
    }

    int64_t result = 0;
    for (const auto &[beam, count] : timelines)
        result += count;

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
