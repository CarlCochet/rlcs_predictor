from glicko import Glicko


class Team(Glicko):

    def __init__(self, name: str):
        super().__init__()
        self.players = []
        self.name = name
