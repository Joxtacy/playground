#include <iostream>
#include <time.h>
#include <random>

int main()
{
  srand(time(NULL));

  int num = rand() % 100 + 1;
  int guess;
  int tries = 1;

  std::cout << "I'm thinking of a number between 1-100.\n";
  std::cout << "Take a guess: ";
  std::cin >> guess;

  while (guess != num && tries < 20) {
    std::cout << "Incorrect. ";

    if (guess < num) {
      std::cout << "Too low. ";
    } else {
      std::cout << "Too high. ";
    }
    std::cout << "Please try again: ";
    std::cin >> guess;
    tries++;
  }

  if (guess == num) {
    std::cout << "Congratulations! You guessed it in " << tries << " tries.\n";
  } else {
    std::cout << "You lose! The correct number was " << num << ".\n";
  }
}
