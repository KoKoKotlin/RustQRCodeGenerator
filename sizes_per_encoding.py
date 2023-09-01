

with open("sizes_per_encoding.txt", "r") as f:
    L, M, Q, H = [], [], [], []

    get_elems = lambda line: line.strip().split()

    for i, line in enumerate(f):
        if i == 0: continue

        elems = get_elems(line)
        version, encoding = elems[0].split("-")

        # only 1 group
        if len(elems) == 8:
            number_of_codewords = elems[1]
            ec_cw_per_block = elems[2]
            number_of_blocks_1 = elems[3]
            number_of_cw_in_1 = elems[4]
            data = [ec_cw_per_block, number_of_blocks_1, number_of_cw_in_1, "0", "0"]

            if encoding == "L": L.append(data)
            elif encoding == "M": M.append(data)
            elif encoding == "Q": Q.append(data)
            elif encoding == "H": H.append(data)
        # 2 groups
        elif len(elems) == 12:
            ec_cw_per_block = elems[2]
            number_of_blocks_1 = elems[3]
            number_of_cw_in_1 = elems[4]
            number_of_blocks_2 = elems[5]
            number_of_cw_in_2 = elems[6]

            data = [ec_cw_per_block, number_of_blocks_1, number_of_cw_in_1, number_of_blocks_2, number_of_cw_in_2]

            if encoding == "L": L.append(data)
            elif encoding == "M": M.append(data)
            elif encoding == "Q": Q.append(data)
            elif encoding == "H": H.append(data)

    for errorCorrection, name in [(L, "L"), (M, "M"), (Q, "Q"), (H, "H")]:
        for j in range(len(errorCorrection)):
            version = j + 1
            print(f"({version}, ErrorCorrectionLevel::{name}, [{','.join(errorCorrection[j])}]),")