"""Functions for creating, transforming, and adding prefixes to strings."""

import re


def add_prefix_un(word: str) -> str:
    """Take the given word and add the 'un' prefix.

    Parameters:
        word (str): The root word.

    Returns:
        str: Root word prepended with 'un'.
    """

    return f"un{word}"


def make_word_groups(vocab_words: list[str]) -> str:
    """Transform a list containing a prefix and words.

    Parameters:
        vocab_words (list[str]): Vocabulary words with prefix at first index.

    Returns:
        str: Prefix followed by vocabulary words with prefix applied.

    This function takes a `vocab_words` list of strings and returns a string
    with the prefix and the words with prefix applied, separated by ' :: '.

    Examples:
        >>> list('en', 'close', 'joy', 'lighten')
        'en :: enclose :: enjoy :: enlighten'.

    """
    vocab_words.insert(1, '')
    prefix, *words = vocab_words
    return " :: ".join(f"{prefix}{word}" for word in words)


def remove_suffix_ness(word: str) -> str:
    """Remove the suffix from the word while keeping spelling in mind.

    Parameters:
        word (str): Word to remove suffix from.

    Returns:
        str: Word with suffix removed & spelling adjusted.

    Examples:
        >>> remove_suffix_ness('heaviness')
        'heavy'

        >>> remove_suffix_ness('sadness')
        'sad'

    """
    return f"{word[:-5]}y" if word.endswith("iness") else word[:-4]


def adjective_to_verb(sentence: str, index: int) -> str:
    """Change the adjective within the sentence to a verb.

    Parameters:
        sentence (str): The word used in a sentence as an adjective.
        index (int): Index of the adjective to remove and transform.

    Returns:
        str: The extracted adjective in verb form.

    Examples:
        >>> adjective_to_verb('It got dark as the sun set.', 2)
        'darken'

        >>> adjective_to_verb('The ink stains her fingers black.', -1)
        'blacken'

    """
    return f"{re.findall(r'\w+', sentence)[index]}en"
