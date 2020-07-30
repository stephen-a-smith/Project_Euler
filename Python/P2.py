a = 1
b = 1
sum = 0

while b < 4000000:
    t = a
    a = b
    b = t + a

    if b % 2 == 0:
        sum = sum + b

print(sum)

