import math
# smallest factor is always prime
def smallest_factor( n ):
    limit = math.ceil(math.sqrt(n))
    for x in range(3, limit, 2):
        if n % x == 0:
            return x
    # n itself is prime
    return n

number = 600851475143
p = 1

while p < number:
    p = smallest_factor(number)
    number = number / p

print(p)
