
@ALPHABET = ['a','b','c','d','e','f','g','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
#puts ALPHABET.size #-> 25
@ALPHABET_SIZE = 25-1

@VOWELS = ['a','e','i','o','u','y'] # is this global constant usage good convention?
#puts VOWELS.size #-> 6
@VOWELS_SIZE = 6-1

@CONSONANTS = ['b','c','d','f','g','j','k','l','m','n','p','q','r','s','t','v','w','x','z']
#puts CONSONANTS.size #-> 19
@CONSONANTS_SIZE = 19-1

def ras(max) #random_arbitrary_sum
  r = 1 + rand(max)
end

def wordmixer_basic(word_length)
  return if word_length < 1

  ch = @ALPHABET[ras(@ALPHABET_SIZE)] # ch returns as a random char
  word = ch # word is now equal one random char

  (word_length-1).times do
    if @CONSONANTS.include? ch
      ch = @VOWELS[ras(@VOWELS_SIZE)]
    else
      ch = @ALPHABET[ras(@ALPHABET_SIZE)]
    end
    word = word + ch
  end
  word
end

puts wordmixer_basic(5) # supposed to return 15 char long word with every other char being vowel and consonant, or multiple vowels