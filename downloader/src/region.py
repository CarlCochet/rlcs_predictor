from glicko import Glicko


class Region(Glicko):

    def __init__(self, name: str):
        super().__init__()
        self.teams = []
        self.name = name
