a = 2
b = 3

# nesting
if True:
    c = a + b
    if c == a + b:
        print(c)

# logical operators, else
if a == 2 and b == 2:
    print(a + b)
else:
    if a == 2 or b == 2:
        print(a)
    print("ne")

# truthy values
if 21:
    print("21 is true")
if "":
    print("empty string is true")
