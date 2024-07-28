import sys

class Card:
    def __init__(self, card_str):
        #self.value
        #self.suit
        value = card_str[:-1]
        suit = card_str[-1]

        match value:
            case 'A': self.value = 1
            case '2': self.value = 2
            case '3': self.value = 3
            case '4': self.value = 4
            case '5': self.value = 5
            case '6': self.value = 6
            case '7': self.value = 7
            case '8': self.value = 8
            case '9': self.value = 9
            case '10': self.value = 10
            case 'J': self.value = 11
            case 'Q': self.value = 12
            case 'K': self.value = 13
            case _:
                print('Unknown Card value: {}'.format(value))
                sys.exit(1)

        if suit not in ('C', 'D', 'H', 'S'):
            print('Unknown Card suit: {}'.format(suit))
            sys.exit(1)
        self.suit = suit

    def to_string(self):
        value = ''
        match self.value:
            case 1: value = 'A'
            case 2: value = '2'
            case 3: value = '3'
            case 4: value = '4'
            case 5: value = '5'
            case 6: value = '6'
            case 7: value = '7'
            case 8: value = '8'
            case 9: value = '9'
            case 10: value = '10'
            case 11: value = 'J'
            case 12: value = 'Q'
            case 13: value = 'K'

        return value + self.suit

    def __eq__(self, other):
        if self.value == other.value and self.suit == other.suit:
            return True
        return False

    def __hash__(self):
        return hash((self.value, self.suit))

    def __str__(self):
        return self.to_string()

    def __repr__(self):
        return self.to_string()
