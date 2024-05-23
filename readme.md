# Poker VRF Activity

### Overview:

The software simulates a fair card game between two players. It leverages cryptography (VRF) to ensure the randomness and unpredictability of each player's card draw, preventing any possibility of cheating.

### Process:

- Key Generation: Each player is assigned a unique cryptographic key, acting as their secret identifier for the game.
- Common Input Agreement: Players agree on a piece of public information, such as a word or phrase, which is used as a starting point for the card selection process - 'PBA VRF Activity' in this case.
- Card Drawing: Using their secret key and the agreed-upon information, each player generates a random number. This number is transformed into a "card" value within a standard deck of cards.
- Winner Determination: The player with the higher card value wins the round. In case of a tie, the round is considered a draw.
