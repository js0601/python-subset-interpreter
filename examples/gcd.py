a = 1200
b = 1024

while b != 0:
    temp = b
    rem = a
    while rem >= b:
        rem = rem - b
    b = rem
    a = temp

print(a)
