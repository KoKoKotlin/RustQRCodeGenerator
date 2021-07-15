

last = 1
print("1, ", end="")
for i in range(1, 256):
    last = (last * 2)
    last = last if last < 256 else last ^ 285
    print(f"{last}, ", end="")
    if i % 10 == 9:
        print()