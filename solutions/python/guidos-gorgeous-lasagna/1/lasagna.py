"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language:
https://en.wikipedia.org/wiki/Guido_van_Rossum

This is a module docstring, used to describe the functionality
of a module and its functions and/or classes.
"""


EXPECTED_BAKE_TIME: int = 40
PREPARATION_TIME: int = 2


def bake_time_remaining(elapsed_bake_time: int) -> int:
    """Calculate the bake time remaining.

    Parameters:
        elapsed_bake_time (int): The baking time already elapsed.

    Returns:
        int: The remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time

def preparation_time_in_minutes(number_of_layers: int) -> int:
    """Calculate the preparation time per layer.
    Parameters:
        number_of_layers (int): The number of layers on the lasagna.

    Returns:
        int: The preparation time (int minutes) derived from `PREPARATION_TIME`.

    Function that takes the number of layers the lasagna has as an argument
    and returns how many minutes it should take to prepare based on `PREPARATION_TIME`.
    """

    return number_of_layers * PREPARATION_TIME


def elapsed_time_in_minutes(number_of_layers: int, elapsed_bake_time: int) -> int:
    """Calculate the time spent making the lasagna.

    Parameters:
        number_of_layers (int): The number of layers on the lasagna.
        elapsed_bake_time (int): The baking time already elapsed.

    Returns:
        int: The amount of time (in minutes) spent preparing the lasagna

    Function that takes the number of layers of the lasagna and the 
    actual minutes the lasagna has been in the oven as arguments
    and returns the total amount of time spent making the lasagna
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
