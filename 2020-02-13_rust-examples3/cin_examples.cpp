#include <iostream>

int main()
{
    std::cout << "Number: ";
    unsigned short number;
    std::cin >> number;

    if (std::cin.fail())
    {
        std::cout << "cin was bad\n";
    }

    std::cout << "Number was \"" << number << "\"\n";
    return 0;
}