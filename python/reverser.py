
text = input("Type a sentence: ").strip().split(' ')

text.reverse()

reversed_sentence = ''

for word in text:
    reversed_sentence += shuffle(word) + ' '
    
    
def shuffle(word):
    size = len(word)
    if size == 1:
        return word
    i = 1
    shuffled = ''
    while i <= size:
        shuffled += word[-i]
        i += 1
    return shuffled
    

reversed_sentence.strip()

print(reversed_sentence)
