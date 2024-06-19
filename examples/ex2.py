if True:
    print("True")

a = 3
if a == 2:
    print("a ist 2")
    a = "jetzt nicht mehr"
else:
    print("a ist nicht 2")
    a = "was anderes"

b = a + " hehe"
if b == "jetzt nicht mehr hehe":
    print(b)
else:
    print(a)
