def is_palindrome( n ):
    divisor = 1
    while n / divisor >= 10:
        divisor *= 10   # set divisor to match size of number

    while n != 0:
        msd = n // divisor # most signifigant digit
        lsd = n % 10 # least signifigant digit

        if lsd != msd:
            return False

        n = (n % divisor) // 10 # strip msd and lsd
        divisor //= 100 # decrease divisor by factor of 10 (2 digits)
    return True

largest = 0

for i in range(100, 1000):
    for j in range(i, 1000): # optimization, reduce duplicate calculations
        temp = i*j
        if is_palindrome(temp) and temp > largest:
            largest = temp

print(largest)
