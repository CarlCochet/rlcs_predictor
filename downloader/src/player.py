import math


class Player:

    def __init__(self, name: str):
        self.rating = 1500
        self.name = name

    def update_rating(self, change: int, score: int, team_scores: [int]):
        gamma = 6
        team_score = sum(team_scores)
        score_ratio = score / team_score
        if change > 0:
            sa = 3 * (math.pow(score_ratio, gamma) /
                      (math.pow(team_scores[0] / team_score, gamma) +
                       math.pow(team_scores[1] / team_score, gamma) +
                       math.pow(team_scores[2] / team_score, gamma)))
        else:
            sa = 3 * (1 / math.pow(score_ratio, gamma) /
                      (1 / (math.pow(team_scores[0] / team_score, gamma)) +
                       (1 / math.pow(team_scores[1] / team_score, gamma)) +
                       (1 / math.pow(team_scores[2] / team_score, gamma))))
        self.rating += change * sa
