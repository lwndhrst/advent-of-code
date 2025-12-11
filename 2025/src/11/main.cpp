#include <cstdint>
#include <fstream>
#include <iostream>
#include <queue>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

#define INPUT_PATH "src/11/input.txt"

// Assuming no cycles in the graph

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

struct node {
    std::string label;
    std::vector<size_t> children;
    size_t num_incoming_edges;

    node(const std::string &label)
    {
        this->label = label;
        this->children = {};
        this->num_incoming_edges = 0;
    }
};

size_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<node> nodes;
    std::unordered_map<std::string, size_t> label_to_index;

    while (std::getline(input, line))
    {
        std::string source = line.substr(0, 3);
        std::vector<std::string> outputs = split(line.substr(5), ' ');

        if (label_to_index.find(source) == label_to_index.end())
        {
            label_to_index[source] = nodes.size();
            nodes.emplace_back(node(source));
        }

        for (const std::string &output : outputs)
        {
            if (label_to_index.find(output) == label_to_index.end())
            {
                label_to_index[output] = nodes.size();
                nodes.emplace_back(node(output));
            }

            size_t src = label_to_index[source];
            size_t out = label_to_index[output];

            nodes[src].children.push_back(label_to_index[output]);
            nodes[out].num_incoming_edges++;
        }
    }

    input.close();

    // Topologial sort? Kahn's algorithm?

    std::queue<size_t> queue;
    for (size_t i = 0; i < nodes.size(); ++i)
        if (nodes[i].num_incoming_edges == 0)
            queue.push(i);

    std::vector<size_t> topo_order;
    while (!queue.empty())
    {
        size_t node = queue.front();
        queue.pop();
        topo_order.push_back(node);

        for (const size_t &child : nodes[node].children)
        {
            nodes[child].num_incoming_edges--;

            if (nodes[child].num_incoming_edges == 0)
                queue.push(child);
        }
    }

    size_t src = label_to_index["you"];
    size_t dst = label_to_index["out"];

    std::vector<size_t> paths(nodes.size(), 0);
    paths[src] = 1;

    for (const size_t &node : topo_order)
        for (const size_t &child : nodes[node].children)
            paths[child] += paths[node];

    return paths[dst];
}

size_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    std::vector<node> nodes;
    std::unordered_map<std::string, size_t> label_to_index;

    while (std::getline(input, line))
    {
        std::string source = line.substr(0, 3);
        std::vector<std::string> outputs = split(line.substr(5), ' ');

        if (label_to_index.find(source) == label_to_index.end())
        {
            label_to_index[source] = nodes.size();
            nodes.emplace_back(node(source));
        }

        for (const std::string &output : outputs)
        {
            if (label_to_index.find(output) == label_to_index.end())
            {
                label_to_index[output] = nodes.size();
                nodes.emplace_back(node(output));
            }

            size_t src = label_to_index[source];
            size_t out = label_to_index[output];

            nodes[src].children.push_back(label_to_index[output]);
            nodes[out].num_incoming_edges++;
        }
    }

    input.close();

    // Topologial sort? Kahn's algorithm?

    std::queue<size_t> queue;
    for (size_t i = 0; i < nodes.size(); ++i)
        if (nodes[i].num_incoming_edges == 0)
            queue.push(i);

    std::vector<size_t> topo_order;
    while (!queue.empty())
    {
        size_t node = queue.front();
        queue.pop();
        topo_order.push_back(node);

        for (const size_t &child : nodes[node].children)
        {
            nodes[child].num_incoming_edges--;

            if (nodes[child].num_incoming_edges == 0)
                queue.push(child);
        }
    }

    // Split into multiple path counting stages
    size_t src = label_to_index["svr"];
    size_t dac = label_to_index["dac"];
    size_t fft = label_to_index["fft"];
    size_t dst = label_to_index["out"];

    const auto &count_paths = [&](size_t from, size_t to) {
        std::vector<size_t> paths(nodes.size(), 0);
        paths[from] = 1;

        for (const size_t &node : topo_order)
            for (const size_t &child : nodes[node].children)
                paths[child] += paths[node];

        return paths[to];
    };

    size_t paths_dac_fft = count_paths(src, dac) * count_paths(dac, fft) * count_paths(fft, dst);
    size_t paths_fft_dac = count_paths(src, fft) * count_paths(fft, dac) * count_paths(dac, dst);

    return paths_dac_fft + paths_fft_dac;
}

int
main()
{
    std::cout << "part one: " << part_one() << std::endl;
    std::cout << "part two: " << part_two() << std::endl;

    return 0;
}
