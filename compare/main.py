from random import randint

secret_number = randint(1, 101)
print('Secret number: ', secret_number)


while True:
    guess = input(str('Please input your guess: '))

    try:
        guess = int(guess)
    except:
        continue

    print('You guessed ', guess)
    
    if guess == secret_number:
        print('{}You guessed it!{}'.format('\x1b[32m', '\x1b[0m'))
        break
    elif guess > secret_number:
        print('Too big!')
    elif guess < secret_number:
        print('Too small!')