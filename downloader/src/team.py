from glicko import Glicko


class Team(Glicko):

    def __init__(self):
        super().__init__()
        self.players = []
