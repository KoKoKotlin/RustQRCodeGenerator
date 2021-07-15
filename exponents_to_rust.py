

with open("exponents.txt", "r") as f:

    lookup1 = list()
    lookup2 = list()

    for line in f:
        get_elems = lambda line: line.strip().split()


        x, y, z, w = get_elems(line)
        lookup1.append((x, y))
        lookup2.append((z, w))


    for l in [lookup1, lookup2]:
        for i, elem in enumerate(l):
            x, y = elem
            print(f"{y}, ", end="")
            if i % 10 == 9: print()

        print("\n\n\n")