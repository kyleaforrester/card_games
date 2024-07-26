#!/usr/bin/env python3

import random
import math

def count_puddings(player_history):
  count = 0
  for i in range(len(player_history)):
    count += player_history[i].count('Pudding')
  return count

def count_maki_rolls(player_cards):
  maki_rolls = []
  for hand in player_cards:
    maki_strings = list(filter(lambda x: 'Maki roll' in x, hand))
    count = 0
    for string in maki_strings:
      count += int(string[0])
    maki_rolls.append(count)
  return maki_rolls

def score_hand(player_cards, hand_index):
  score = 0
  #Score Maki Rolls
  maki_rolls = count_maki_rolls(player_cards)
  max_ties = 0
  sec_ties = 0
  max_maki_roll = max(maki_rolls)
  for i in range(len(maki_rolls)):
    if (maki_rolls[i] == max_maki_roll):
      max_ties += 1
  if (maki_rolls[hand_index] == max_maki_roll):
    score += 6//max_ties
  if (max_ties == 1):
    sec_maki_roll = sorted(set(maki_rolls), reverse=True)[1]
    for i in range(len(maki_rolls)):
      if (maki_rolls[i] == sec_maki_roll):
        sec_ties += 1
    if (maki_rolls[hand_index] == sec_maki_roll):
      score += 3//sec_ties
  #Score Tempura
  tempura_count = player_cards[hand_index].count('Tempura')
  tempura_pairs = tempura_count//2
  score += 5*tempura_pairs
  #Score Sashimi
  sashimi_count = player_cards[hand_index].count('Sashimi')
  sashimi_pairs = sashimi_count//3
  score += 10*sashimi_pairs
  #Score Dumplings
  dumpling_count = player_cards[hand_index].count('Dumpling')
  if (dumpling_count == 1):
    score += 1
  elif (dumpling_count == 2):
    score += 3
  elif (dumpling_count == 3):
    score += 6
  elif (dumpling_count == 4):
    score += 10
  elif (dumpling_count >= 5):
    score += 15
  #Score Nigiri/Wasabi
  my_hand = player_cards[hand_index]
  for i in range(len(my_hand)):
    if ('Nigiri' in my_hand[i]):
      if (my_hand[i] == 'Egg Nigiri'):
        nigiri_score = 1
      elif (my_hand[i] == 'Salmon Nigiri'):
        nigiri_score = 2
      elif (my_hand[i] == 'Squid Nigiri'):
        nigiri_score = 3
      preceded_by_wasabi = False
      if (i > 0):
        pre_slice = my_hand[i-1::-1]
        for j in range(len(pre_slice)):
          if ('Nigiri' in pre_slice[j]):
            preceded_by_wasabi = False
            break
          elif ('Wasabi' == pre_slice[j]):
            preceded_by_wasabi = True
            break
      if (preceded_by_wasabi):
        score += nigiri_score * 3
      else:
        score += nigiri_score
  return score

def pretty_string_history(game_history, scoreboard):
  my_string = ''
  for i in range(len(game_history)):
    my_string += 'Player {}: {} points\n'.format(i, sum(scoreboard[i]))
    for j in range(len(game_history[i])):
      my_string += '  Round {}: {} points: {}\n'.format(j, game_history[i][j], scoreboard[i][j])
    my_string += '\n'
  return my_string

def pretty_string_round(player_cards):
  my_string = ''
  for i in range(len(player_cards)):
    my_string += 'Player {}: {}\n'.format(i, player_cards[i])
  my_string += '\n'
  return my_string

def select_card(hand, player_cards, game_history, full_deck, player_index):
  p_cards = player_cards[player_index]
  #Sashimi
  current_sashimi = p_cards.count('Sashimi')%3
  sashimi_bias = (len(hand)/4.0)**2
  sashimi_score = 0
  if (current_sashimi == 0):
    sashimi_score = min(sashimi_bias, (10/3))
  elif (current_sashimi == 1):
    if (len(hand) == 1):
      sashimi_score = 0
    else:
      sashimi_score = min(len(hand), (20/3))
  elif (current_sashimi == 2):
    sashimi_score = 10
  #Tempura
  current_tempura = p_cards.count('Tempura')%2
  tempura_bias = (len(hand)/2)**2
  tempura_score = 0
  if (current_tempura == 0):
    tempura_score = min(tempura_bias, (5/2))
  elif (current_tempura == 1):
    tempura_score = 5
  #Dumplings
  current_dumplings = p_cards.count('Dumpling')
  dumpling_bias = (len(hand)/5)+(current_dumplings + 1)
  dumpling_score = 0
  if (current_dumplings == 0):
    dumpling_score = min(dumpling_bias, (15/5))
  elif (current_dumplings == 1):
    dumpling_score = min(dumpling_bias, (14/4))
  elif (current_dumplings == 2):
    dumpling_score = min(dumpling_bias, (12/3))
  elif (current_dumplings == 3):
    dumpling_score = min(dumpling_bias, (9/2))
  elif (current_dumplings == 4):
    dumpling_score = 5
  else:
    dumpling_score = 0
  #Wasabi
  current_wasabi = p_cards.count('Wasabi')
  wasabi_score = 0
  if (current_wasabi > 0 or len(hand) == 1):
    wasabi_score = 0
  else:
    salmon_chance = (98/108)**(len(hand)-1)
    squid_chance = (103/108)**(len(hand)-1)
    egg_chance = (103/108)**(len(hand)-1)
    wasabi_score = (1-squid_chance)*salmon_chance*3 + squid_chance*(9/2) + (1-squid_chance)*(1-salmon_chance)*egg_chance*(3/2)
  #Nigiri
  preceded_by_wasabi = False
  reverse_p_cards = p_cards[::-1]
  for card in reverse_p_cards:
    if ('Nigiri' in card):
      preceded_by_wasabi = False
      break
    elif ('Wasabi' == card):
      preceded_by_wasabi = True
      break
  wasabi_multiplier = 1
  if (preceded_by_wasabi):
    wasabi_multiplier = 3
  squid = 3*wasabi_multiplier
  salmon = 2*wasabi_multiplier
  egg = wasabi_multiplier
  #Maki Rolls
  maki_strings = list(filter(lambda x: 'Maki roll' in x, p_cards))
  current_maki_rolls = 0
  for string in maki_strings:
    current_maki_rolls += int(string[0])
  maki_roll_bias = 2**(-((current_maki_rolls-3)**2)/16)
  #Pudding
  pudding_score = 1.9

  #Score cards
  card_dict = {}
  for card in hand:
    if ('Maki roll' in card):
      count = int(card[0])
      card_dict[card] = maki_roll_bias*count
    elif ('Tempura' == card):
      card_dict[card] = tempura_score
    elif ('Sashimi' == card):
      card_dict[card] = sashimi_score
    elif ('Dumpling' == card):
      card_dict[card] = dumpling_score
    elif ('Nigiri' in card):
      if ('Squid' in card):
        card_dict[card] = squid
      elif ('Salmon' in card):
        card_dict[card] = salmon
      else:
        card_dict[card] = egg
    elif ('Wasabi' == card):
      card_dict[card] = wasabi_score
    elif ('Pudding' == card):
      card_dict[card] = pudding_score
  card_dict_list = sorted(list(card_dict.items()), key=lambda x: x[1], reverse=True)
  return card_dict_list[0][0]
  
def select_human_card(hand, player_cards, game_history, scoreboard):
  correct_responses = ['scoreboard', 'round', 'hint'] + hand
  prompt = '''\nType in a command or the card you want:
Commands:
    scoreboard
    round
    hint
Cards:\n'''
  for i in hand:
    prompt += '    ' + i + '\n'
  prompt += 'What would you like? '
  response = input(prompt)
  while response not in hand:
    if (response == 'scoreboard'):
      print(pretty_string_history(game_history,scoreboard))
    elif (response == 'round'):
      print(pretty_string_round(player_cards))
    elif (response == 'hint'):
      print('Computer recommendation: {}'.format(select_card(hand, player_cards, game_history, [], 0)))
    else:
      print('Invalid Command')
    response = input(prompt)
  print()
  return response

def main():
  #Randomly generate deck
  player_count = int(input('How many players? '))
  deck = ['Tempura']*14 + ['Sashimi']*14 + ['Dumpling']*14 + ['2 Maki rolls']*12 + ['3 Maki rolls']*8 + ['1 Maki roll']*6 + ['Salmon Nigiri']*10 + ['Squid Nigiri']*5 + ['Egg Nigiri']*5 + ['Pudding']*11 + ['Wasabi']*6# + ['Chopsticks']*4
  full_deck = [card for card in deck]
  random.shuffle(deck)

  #game_history is a 3 dimensional list.  Player level, Round level, Card level
  game_history = [[] for i in range(player_count)]
  #scoreboard is a 2 dimensional list.  Player level, Round level (Puddings are 4th element in round level)
  scoreboard = [[] for i in range(player_count)]
  #player_cards is a 2 dimensional list.  Player level, card level.
  #Contains played cards for the current round
  player_cards = [[] for i in range(player_count)]

  #3 rounds, passing left
  for i in range(3):

    #Create hands
    init_hand_size = 12-player_count
    hands = []
    for i in range(player_count):
      hands.append(deck[:init_hand_size])
      deck = deck[init_hand_size:]

    #Loop drawing cards until there are no more cards in hands
    while (len(hands[0]) > 0):
      face_down_cards = []
      #Select each player's card and place face down
      for i in range(player_count):
        if (i == 0):
          card = select_human_card(hands[i], player_cards, game_history, scoreboard)
        else:
          card = select_card(hands[i], player_cards, game_history, full_deck, i)
        face_down_cards.append(card)
    
      #Remove face down cards from hands and add to player_cards
      for i in range(player_count):
        hands[i].remove(face_down_cards[i])
        player_cards[i].append(face_down_cards[i])

      #Rotate hands to the left
      temp = [card for card in hands[0]]
      for i in range(player_count-1):
        hands[i] = [card for card in hands[i+1]]
      hands[-1] = temp

    #Round is over
    #Update scores
    for i in range(player_count):
      scoreboard[i].append(score_hand(player_cards, i))
    #Add player_cards to history
    for i in range(player_count):
      game_history[i].append(player_cards[i])
      player_cards[i] = []

  #Game is over
  #Update Pudding scores
  puddings = []
  for i in range(player_count):
    puddings.append(count_puddings(game_history[i]))
  for i in range(player_count):
    game_history[i].append(['Pudding']*puddings[i])
  max_ties = 0
  min_ties = 0
  max_pudding = max(puddings)
  min_pudding = min(puddings)
  for i in range(player_count):
    if (puddings[i] == max_pudding):
      max_ties += 1
    if (puddings[i] == min_pudding):
      min_ties += 1
  for i in range(player_count):
    if (puddings[i] == max_pudding):
      scoreboard[i].append(6//max_ties)
    if (puddings[i] == min_pudding):
      scoreboard[i].append(-(6//min_ties))
    if (puddings[i] > min_pudding and puddings[i] < max_pudding):
      scoreboard[i].append(0)

  print(pretty_string_history(game_history, scoreboard))
  max_score = 0
  player_index = 0
  for i in range(player_count):
    player_score = sum(scoreboard[i])
    if (player_score > max_score):
      max_score = player_score
      player_index = i
  print('Congratulations to Player {}!!'.format(player_index))

if __name__=='__main__':
  main()
