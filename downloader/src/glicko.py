"""
Copyright (c) 2009 Ryan Kirkman
Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:
The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
"""

import math


class Glicko:
    _tau = 0.5

    def get_rating(self):
        return (self.__rating * 173.7178) + 1500

    def set_rating(self, rating):
        self.__rating = (rating - 1500) / 173.7178

    rating = property(get_rating, set_rating)

    def get_rd(self):
        return self.__rd * 173.7178

    def set_rd(self, rd):
        self.__rd = rd / 173.7178

    rd = property(get_rd, set_rd)

    def __init__(self, rating=1500, rd=350, vol=0.06):
        # For testing purposes, preload the values assigned to an unrated player.
        self.__rating = rating
        self.__rd = rd
        self.set_rating(rating)
        self.set_rd(rd)
        self.vol = vol

    def _pre_rating_rd(self):
        """ Calculates and updates the player's rating deviation for the beginning of a rating period.

        _pre_rating_rd() -> None
        """
        self.__rd = math.sqrt(math.pow(self.__rd, 2) + math.pow(self.vol, 2))

    def update_player(self, rating_list, rd_list, outcome_list):
        """ Calculates the new rating and rating deviation of the player.

        update_player(list[int], list[int], list[bool]) -> None
        """
        # Convert the rating and rating deviation values for internal use.
        rating_list = [(x - 1500) / 173.7178 for x in rating_list]
        rd_list = [x / 173.7178 for x in rd_list]

        v = self._v(rating_list, rd_list)
        self.vol = self._new_vol(rating_list, rd_list, outcome_list, v)
        self._pre_rating_rd()

        self.__rd = 1 / math.sqrt((1 / math.pow(self.__rd, 2)) + (1 / v))

        temp_sum = 0
        for i in range(len(rating_list)):
            temp_sum += self._g(rd_list[i]) * \
                        (outcome_list[i] - self._E(rating_list[i], rd_list[i]))
        self.__rating += math.pow(self.__rd, 2) * temp_sum

    # step 5
    def _new_vol(self, rating_list, rd_list, outcome_list, v):
        """ Calculating the new volatility as per the Glicko2 system.

        _new_vol(list, list, list, float) -> float
        """
        # step 1
        a = math.log(self.vol ** 2)
        eps = 0.000001
        A = a

        # step 2
        B = None
        delta = self._delta(rating_list, rd_list, outcome_list, v)
        tau = self._tau
        if (delta ** 2) > ((self.__rd ** 2) + v):
            B = math.log(delta ** 2 - self.__rd ** 2 - v)
        else:
            k = 1
            while self._f(a - k * math.sqrt(tau ** 2), delta, v, a) < 0:
                k = k + 1
            B = a - k * math.sqrt(tau ** 2)

        # step 3
        fA = self._f(A, delta, v, a)
        fB = self._f(B, delta, v, a)

        # step 4
        while math.fabs(B - A) > eps:
            # a
            C = A + ((A - B) * fA) / (fB - fA)
            fC = self._f(C, delta, v, a)
            # b
            if fC * fB < 0:
                A = B
                fA = fB
            else:
                fA = fA / 2.0
            # c
            B = C
            fB = fC

        # step 5
        return math.exp(A / 2)

    def _f(self, x, delta, v, a):
        ex = math.exp(x)
        num1 = ex * (delta ** 2 - self.__rating ** 2 - v - ex)
        denom1 = 2 * ((self.__rating ** 2 + v + ex) ** 2)
        return (num1 / denom1) - ((x - a) / (self._tau ** 2))

    def _delta(self, rating_list, rd_list, outcome_list, v):
        """ The delta function of the Glicko2 system.

        _delta(list, list, list) -> float
        """
        temp_sum = 0
        for i in range(len(rating_list)):
            temp_sum += self._g(rd_list[i]) * (outcome_list[i] - self._E(rating_list[i], rd_list[i]))
        return v * temp_sum

    def _v(self, rating_list, rd_list):
        """ The v function of the Glicko2 system.

        _v(list[int], list[int]) -> float
        """
        temp_sum = 0
        for i in range(len(rating_list)):
            temp_E = self._E(rating_list[i], rd_list[i])
            temp_sum += math.pow(self._g(rd_list[i]), 2) * temp_E * (1 - temp_E)
        return 1 / temp_sum

    def _E(self, p2rating, p2rd):
        """ The Glicko E function.

        _E(int) -> float
        """
        return 1 / (1 + math.exp(-1 * self._g(p2rd) * (self.__rating - p2rating)))

    def _g(self, rd):
        """ The Glicko2 g(RD) function.

        _g() -> float
        """
        return 1 / math.sqrt(1 + 3 * math.pow(rd, 2) / math.pow(math.pi, 2))

    def did_not_compete(self):
        """ Applies Step 6 of the algorithm. Use this for players who did not compete in the rating period.

        did_not_compete() -> None
        """
        self._pre_rating_rd()
