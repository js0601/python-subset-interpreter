def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1

    return fib(n - 1) + fib(n - 2)

def fib_list(n):
    result = []
    while n >= 0:
        result = [fib(n)] + result
        n = n - 1
    return result

print(fib_list(9))
