#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

#include <z3++.h>

#define INPUT_PATH "src/10/input.txt"

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

std::string
vec_to_string(const std::vector<size_t> vec)
{
    std::string result;

    for (size_t i = 0; i < vec.size(); ++i)
    {
        result.append(std::to_string(vec[i]));
        if (i < vec.size() - 1)
            result.push_back(',');
    }

    return result;
}

std::vector<size_t>
vec_from_string(const std::string &str)
{
    std::vector<size_t> result;

    for (const std::string &s : split(str, ','))
        result.push_back(std::stoul(s));

    return result;
}

struct machine {
    std::string lights;
    std::vector<std::vector<size_t>> buttons;
    std::string joltages;

    machine(const std::string &line)
    {
        size_t lights_end = line.find_first_of(' ');
        size_t buttons_end = line.find_last_of(' ');

        std::string lights_str = line.substr(0, lights_end);
        std::string buttons_str = line.substr(lights_end + 1, buttons_end - lights_end);
        std::string joltages_str = line.substr(buttons_end + 1);

        // trim '[' and ']'
        lights = lights_str.substr(1, lights_str.size() - 2);

        for (size_t i = 0; i < buttons_str.size(); ++i)
        {
            if (buttons_str[i] == ')' || buttons_str[i] == ' ')
                continue;

            if (buttons_str[i] == '(')
            {
                size_t button_end = buttons_str.find(')', i);

                // trim '(' and ')'
                std::string button = buttons_str.substr(i + 1, button_end - i - 1);
                buttons.push_back(vec_from_string(button));

                i = button_end + 1;
            }
        }

        // trim '{' and '}'
        joltages = joltages_str.substr(1, joltages_str.size() - 2);
    }

    std::unordered_set<std::string> get_possible_next_light_states(const std::string &state)
    {
        std::unordered_set<std::string> result;

        for (const std::vector<size_t> &b : buttons)
        {
            std::string new_state = state;

            for (const size_t &i : b)
                new_state[i] = new_state[i] == '#' ? '.' : '#';

            result.insert(new_state);
        }

        return result;
    }

    std::unordered_set<std::string> get_possible_next_joltage_states(const std::string &state)
    {
        std::unordered_set<std::string> result;

        for (const std::vector<size_t> &b : buttons)
        {
            std::vector<size_t> new_state = vec_from_string(state);

            bool valid = true;

            for (const size_t &i : b)
            {
                if (new_state[i] == 0)
                {
                    valid = false;
                    break;
                }

                --new_state[i];
            }

            if (valid)
                result.insert(vec_to_string(new_state));
        }

        return result;
    }
};

int64_t
part_one()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    int64_t result = 0;

    while (std::getline(input, line))
    {
        machine m(line);

        std::string initial_state(m.lights.size(), '.');
        std::unordered_set<std::string> states = {initial_state};

        size_t button_presses = 0;

        while (states.find(m.lights) == states.end())
        {
            std::unordered_set<std::string> new_states = {};

            for (const auto &state : states)
                for (const auto &new_state : m.get_possible_next_light_states(state))
                    new_states.insert(new_state);

            states = new_states;
            ++button_presses;
        }

        result += button_presses;
    }

    input.close();

    return result;
}

int64_t
part_two()
{
    std::ifstream input;
    std::string line;

    input.open(INPUT_PATH);

    int64_t result = 0;

    while (std::getline(input, line))
    {
        machine m(line);

        std::vector<size_t> joltages = vec_from_string(m.joltages);

        z3::context ctx = {};
        z3::optimize opt(ctx);

        z3::expr_vector button_presses(ctx);

        for (size_t i = 0; i < m.buttons.size(); ++i)
        {
            button_presses.push_back(ctx.int_const(std::to_string(i).c_str()));
            opt.add(button_presses[i] >= 0);
        }

        for (size_t i = 0; i < joltages.size(); ++i)
        {
            z3::expr_vector affecting_buttons(ctx);

            for (size_t j = 0; j < m.buttons.size(); ++j)
            {
                const std::vector<size_t> &button = m.buttons[j];
                for (size_t k = 0; k < button.size(); ++k)
                {
                    if (button[k] == i)
                    {
                        affecting_buttons.push_back(button_presses[j]);
                        break;
                    }
                }
            }

            if (!affecting_buttons.empty())
                opt.add(z3::sum(affecting_buttons) == ctx.int_val(joltages[i]));
        }

        opt.minimize(z3::sum(button_presses));

        if (opt.check() == z3::sat)
        {
            z3::model model = opt.get_model();
            result += model.eval(z3::sum(button_presses), true).as_int64();
        }

        // Giga slow brute force
        // std::vector<size_t> target_state(split(m.joltages, ',').size(), 0);
        // std::string target_state_str = vec_to_string(target_state);

        // std::string initial_state = m.joltages;
        // std::unordered_set<std::string> states = {initial_state};

        // size_t button_presses = 0;

        // while (states.find(target_state_str) == states.end())
        // {
        //     std::unordered_set<std::string> new_states = {};

        //     for (const auto &state : states)
        //         for (const auto &new_state : m.get_possible_next_joltage_states(state))
        //             new_states.insert(new_state);

        //     states = new_states;
        //     ++button_presses;
        // }

        // std::cout << button_presses << std::endl;

        // result += button_presses;
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
