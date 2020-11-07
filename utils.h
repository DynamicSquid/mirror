#pragma once

#include <iostream>
#include <string>

std::string input()
{
    std::string uinput;
    getline(std::cin, uinput);

    return uinput;
}