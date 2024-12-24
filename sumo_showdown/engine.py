import card
import board
import copy

def evaluate_card(card, enemy_cards):
    if card.suit == 'C':
        enemy_cards_better = len(list(filter(lambda x: (x.suit == 'C' and x.value > card.value) or (x.suit == 'S' and x.value >= card.value), enemy_cards)))
        # The floor is the chance the opponent does not rest (2/3) and plays no Push or Throw (1/4), which is (1/6).  Floor is (1/6 + 1/3) = 1/2. Exponential degrading
        value = ((1 / (2**enemy_cards_better)) * 1/2) + (1/2)
    elif card.suit == 'S':
        enemy_cards_better = len(list(filter(lambda x: (x.suit == 'C' and x.value > card.value) or (x.suit == 'S' and x.value > card.value), enemy_cards)))
        # Floor is the chance the opponent rests (1/3). Negating a Rest is the same as negating a 1.5 card play by the opponent, so the Floor is 1.5*(1/3) = 1/2. Exponential degrading
        value = ((1 / (2**enemy_cards_better)) * 1/2) + (1/2)
    elif card.suit == 'H':
        enemy_cards_better = len(list(filter(lambda x: x.suit == 'H' and x.value > card.value, enemy_cards)))
        # Floor is the chance the opponent does not rest (2/3) and plays no Slap (9/16), which makes the floor (1/3 + 3/8) = 17/24 and ceiling 1. Since Push is slightly stronger, make the ceiling 3/4. Exponential degrading
        value = (3/4) * (((1 / (2**enemy_cards_better)) * (7/24)) + (17/24))
    elif card.suit == 'D':
        enemy_cards_better = len(list(filter(lambda x: x.suit == 'D' and x.value > card.value, enemy_cards)))
        # Diamonds have the same floor as Slap, 17/24 and ceiling 1. Salt is slightly weaker, so decrease by 3/4. Exponential degrading
        value = (3/4) * (((1 / (2**enemy_cards_better)) * (7/24)) + (17/24))
    return value

def evaluate_board(board):
    # Always given from Player a's perspective
    if board.a_win:
        return 1
    elif board.b_win:
        return 0

    # If about to be Thrown
    if len(board.a_hand) == 0 and any(map(lambda x: x.suit == 'S', board.b_hand)):
        return 0
    if len(board.b_hand) == 0 and any(map(lambda x: x.suit == 'S', board.a_hand)):
        return 1

    # If about to be Pushed out
    a_pushes = sorted([x.value for x in board.a_hand if x.suit == 'C'])
    b_pushes = sorted([x.value for x in board.b_hand if x.suit == 'C'])
    a_best_pushes = 0
    b_best_pushes = 0
    if len(a_pushes) >= 2:
        a_best_pushes = sum(a_pushes[-2:])
    elif len(a_pushes) >= 1:
        a_best_pushes = a_pushes[-1]

    if len(b_pushes) >= 2:
        b_best_pushes = sum(b_pushes[-2:])
    elif len(b_pushes) >= 1:
        b_best_pushes = b_pushes[-1]

    a_throws = sorted([x.value for x in board.a_hand if x.suit == 'S'])
    b_throws = sorted([x.value for x in board.b_hand if x.suit == 'S'])
    a_best_throws = 0
    b_best_throws = 0
    if len(a_throws) >= 2:
        a_best_throws = sum(a_throws[-2:])
    elif len(a_throws) >= 1:
        a_best_throws = a_throws[-1]

    if len(b_throws) >= 2:
        b_best_throws = sum(b_throws[-2:])
    elif len(b_throws) >= 1:
        b_best_throws = b_throws[-1]

    if board.position == 0 and (max(a_best_pushes, a_best_throws) < b_best_pushes or (len(b_pushes) > 0 and len(b_throws) > 0 and max(a_best_pushes, a_best_throws) < max(b_throws))):
        return 0
    if board.position == 2 and (max(b_best_pushes, b_best_throws) < a_best_pushes or (len(a_pushes) > 0 and len(a_throws) > 0 and max(b_best_pushes, b_best_throws) < max(a_throws))):
        return 1

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

    b_value_spent = b_discard_value / (b_hand_value + b_discard_value)

    # New evaluation below.  Set the hand exhaustion to be as important as the push counter
    # Normalize [-2, 4] between [0, 1]
    new_position = board.position - 2*a_value_spent + 2*b_value_spent
    return (new_position + 2) / 6

    # Each Rest is considered equal to 1.5 Pushes
    # Calculate how far each player has depleted their hand and needs Rests
    # Add the needed Rests onto the push position. Since max range of 2 Rests and the position is initially between [0, 2], after adding the Rest the position becomes [-3, 5]
    # Take a linear value between [-3, 5] to squash the result between 0 and 1

    # Take the average of value and count to determine the percentage of depletion. Mutiply by the remaining rests required
    a_needed_rests = a_value_spent * (2 - board.a_rests)
    b_needed_rests = b_value_spent * (2 - board.b_rests)
    
    # new_position will be a value between [-3, 5]
    new_position = board.position - (3/2)*a_needed_rests + (3/2)*b_needed_rests

    return (new_position + 3) / 8

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
            if any(map(lambda x: x.suit == 'H', a_moves[a] + b_moves[b])):
                monte_sum = 0
                monte_simulations = 20
                for _ in range(monte_simulations):
                    # Perform Monte Carlo simulation of what the result will be due to random Slap discards
                    temp_board = copy.deepcopy(board)
                    temp_board.resolve_moves(a_moves[a], b_moves[b])
                    monte_sum += evaluate_board(temp_board)
                table[a][b] = monte_sum / monte_simulations
            else:
                temp_board = copy.deepcopy(board)
                temp_board.resolve_moves(a_moves[a], b_moves[b])
                table[a][b] = evaluate_board(temp_board)

    a_weights = [10]*len(a_moves)
    b_weights = [10]*len(b_moves)

    # Add logic to never consider illogical moves
    a_illogical_moves_idx = []
    b_illogical_moves_idx = []

    # Iterate to find approximate Nash equilibriums
    for _ in range(256):
        outcome = calculate_outcome(table, a_weights, b_weights)
        new_a_weights = [a for a in a_weights]
        new_b_weights = [b for b in b_weights]
        for a in range(len(a_weights)):
            if a in a_illogical_moves_idx:
                continue

            a_weight_orig = a_weights[a]
            
            # Try increase and decrease
            for i in [-1, 1]:
                if i == -1 and (sum(new_a_weights) == 1 or sum(a_weights) == 1 or a_weights[a] == 0):
                    continue
                a_weights[a] = a_weight_orig + i
                new_outcome = calculate_outcome(table, a_weights, b_weights)
                if new_outcome > outcome:
                    new_a_weights[a] = a_weights[a]
                    a_weights[a] = a_weight_orig
                    break
                a_weights[a] = a_weight_orig
                
        for b in range(len(b_weights)):
            if b in b_illogical_moves_idx:
                continue

            b_weight_orig = b_weights[b]
            
            # Try increase and decrease
            for i in [-1, 1]:
                if i == -1 and (sum(new_b_weights) == 1 or sum(b_weights) == 1 or b_weights[b] == 0):
                    continue
                b_weights[b] = b_weight_orig + i
                new_outcome = calculate_outcome(table, a_weights, b_weights)
                if new_outcome < outcome:
                    new_b_weights[b] = b_weights[b]
                    b_weights[b] = b_weight_orig
                    break
                b_weights[b] = b_weight_orig

        a_weights = new_a_weights
        b_weights = new_b_weights

    return (a_moves, a_weights, b_moves, b_weights)
