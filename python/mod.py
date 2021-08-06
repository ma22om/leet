
def hardmod(divident, divisor):
    quotient = divident
    modulus = 0

    # replicating division behavior
    while quotient >= divisor:
        quotient -= divisor
        # finding modulus
        if quotient < divisor:
            modulus = quotient
    return modulus

divident = float(input("Enter a divident: "))
divisor = float(input("Enter a divisor: "))
print(hardmod(divident, divisor))
