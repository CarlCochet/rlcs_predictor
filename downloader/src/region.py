from glicko import Glicko


class Region(Glicko):

    def __init__(self):
        super().__init__()
        self.teams = []
