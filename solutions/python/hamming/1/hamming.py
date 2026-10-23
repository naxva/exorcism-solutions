def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("Strands must be of equal length.")
    count = 0
    for (i, first) in enumerate(strand_a):
        second = strand_b[i]
        if first != second:
            count += 1

    return count
        
