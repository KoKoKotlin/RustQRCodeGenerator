

with open("versions.txt", "r") as f:
    L, M, Q, H = [], [], [], []

    get_elems = lambda line: line.strip().split()[1::]

    for i, line in enumerate(f):
        if i % 5 == 0: continue
        if i % 5 == 1: L.append(get_elems(line))
        if i % 5 == 2: M.append(get_elems(line))
        if i % 5 == 3: Q.append(get_elems(line))
        if i % 5 == 4: H.append(get_elems(line))

    for errorCorrection, name in [(L, "L"), (M, "M"), (Q, "Q"), (H, "H")]:
        for j in range(0, 3):
            encoding = ["Numeric", "AlphaNumeric", "Byte"][j]
            print(f"(QREncoding::{encoding}, ErrorCorrectionLevel::{name}, vec![{','.join([errorCorrection[i][j] for i in range(len(errorCorrection))])}]),")