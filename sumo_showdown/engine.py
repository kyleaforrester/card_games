import card
import board
import copy

def evaluate_card(card, enemy_cards):
    if card.suit == 'C':
        enemy_cards_better = len(list(filter(lambda x: (x.suit == 'C' and x.value > card.value) or (x.suit == 'S' and x.value >= card.value), enemy_cards)))
        # The floor is the chance the opponent does not rest (2/3) and plays no Push or Throw (3/4), which is 1/2. Exponential degrading
        value = ((1 / (2**enemy_cards_better)) * 1/2) + (1/2)
    elif card.suit == 'S':
        enemy_cards_better = len(list(filter(lambda x: (x.suit == 'C' and x.value > card.value) or (x.suit == 'S' and x.value > card.value), enemy_cards)))
        enemy_cards_c_s = len(list(filter(lambda x: x.suit in ('C', 'S'), enemy_cards)))
        # No floor, linear degrading
        value = 1 - (enemy_cards_better / enemy_cards_c_s) if enemy_cards_c_s > 0 else 0
    elif card.suit == 'H':
        enemy_cards_better = len(list(filter(lambda x: x.suit == 'H' and x.value > card.value, enemy_cards)))
        # Floor is the chance the opponent does not rest (2/3) and plays no Slap (1/2), which makes the floor 2/3 and ceiling 1. Since Push is slightly weaker, make the ceiling 3/4, so the floor is 1/2. Exponential degrading
        value = (3/4) * (((1 / (2**enemy_cards_better)) * (1/3)) + (2/3))
    elif card.suit == 'D':
        enemy_cards_better = len(list(filter(lambda x: x.suit == 'D' and x.value > card.value, enemy_cards)))
        # Diamonds have the same floor as Slap, 2/3 and ceiling 1. Salt is slightly weaker, so decrease by 2/3. Exponential degrading
        value = (2/3) * (((1 / (2**enemy_cards_better)) * (1/3)) + (2/3))
    return value

def evaluate_board(board):
    # Always given from Player a's perspective
    if board.a_win:
        return 1
    elif board.b_win:
        return 0

    a_cards = board.a_hand + board.a_discard
    b_cards = board.b_hand + board.b_discard

    a_hand_value = 0
    a_discard_value = 0
    for c in a_cards:
        value = evaluate_card(c, b_cards)
        if c in board.a_hand:
            a_hand_value += value
        else:
            a_discard_value += value

    b_hand_value = 0
    b_discard_value = 0
    for c in b_cards:
        value = evaluate_card(c, a_cards)
        if c in board.b_hand:
            b_hand_value += value
        else:
            b_discard_value += value

    a_value_spent = a_discard_value / (a_hand_value + a_discard_value)
    a_count_spent = len(board.a_discard) / (len(board.a_hand) + len(board.a_discard))

    b_value_spent = b_discard_value / (b_hand_value + b_discard_value)
    b_count_spent = len(board.b_discard) / (len(board.b_hand) + len(board.b_discard))

    # Each Rest is considered equal to a Push
    # Calculate how far each player has depleted their hand and needs Rests
    # Add the needed Rests onto the push position. Since max range of 2 Rests and the position is initially between [0, 4], after adding the Rest the position becomes [-2, 6]
    # Take a linear value between [-2, 6] to squash the result between 0 and 1

    # Take the average of value and count to determine the percentage of depletion. Mutiply by the remaining rests required
    a_needed_rests = ((a_value_spent + a_count_spent) / 2) * (2 - board.a_rests)
    b_needed_rests = ((b_value_spent + b_count_spent) / 2) * (2 - board.b_rests)
    
    # new_position will be a value between [-2, 6]
    new_position = board.position - a_needed_rests + b_needed_rests

    return (new_position + 2) / 8

def calculate_outcome(table, a_weights, b_weights):
    a_sum = sum(a_weights)
    b_sum = sum(b_weights)

    outcome = 0
    for a in range(len(table)):
        for b in range(len(table[a])):
            outcome += table[a][b] * (a_weights[a] * b_weights[b]) / (a_sum * b_sum)

    return outcome

def calculate_equilibrium(board):
    moves = board.get_moves()

    a_moves = moves[0]
    b_moves = moves[1]

    table = [[0.5] * len(b_moves) for _ in range(len(a_moves))]

    for a in range(len(table)):
        for b in range(len(table[a])):
            temp_board = copy.deepcopy(board)
            temp_board.resolve_moves(a_moves[a], b_moves[b])
            table[a][b] = evaluate_board(temp_board)

    a_weights = [10]*len(a_moves)
    b_weights = [10]*len(b_moves)

    outcome = calculate_outcome(table, a_weights, b_weights)
    for _ in range(256):
        for a in range(len(a_weights)):
            a_weight_orig = a_weights[a]
            
            # Try increase and decrease
            for i in [-1, 1]:
                if i == -1 and (sum(a_weights) == 1 or a_weights[a] == 0):
                    continue
                a_weights[a] = a_weight_orig + i
                new_outcome = calculate_outcome(table, a_weights, b_weights)

                # Delete after debug
                a_sum = sum(a_weights)
                b_sum = sum(b_weights)
                strategy_outcome = sum([table[a][b] * a_weights[a] * b_weights[b] / (a_sum * b_sum) for b in range(len(b_weights))])
                if new_outcome > outcome:
                    outcome = new_outcome
                    break
                else:
                    a_weights[a] = a_weight_orig
                
        for b in range(len(b_weights)):
            b_weight_orig = b_weights[b]
            
            # Try increase and decrease
            for i in [-1, 1]:
                if i == -1 and (sum(b_weights) == 1 or b_weights[b] == 0):
                    continue
                b_weights[b] = b_weight_orig + i
                new_outcome = calculate_outcome(table, a_weights, b_weights)
                if new_outcome < outcome:
                    outcome = new_outcome
                    break
                else:
                    b_weights[b] = b_weight_orig

    return (a_moves, a_weights, b_moves, b_weights)
