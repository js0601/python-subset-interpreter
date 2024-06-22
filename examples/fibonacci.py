n = 10 # how many fib numbers
a = 0
b = 1

i = 1
while i <= n:
    print(a)
    next = a + b
    a = b
    b = next
    i = i + 1
