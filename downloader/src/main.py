import json

from tqdm import tqdm
from octanegg import Octane

from player import Player
from team import Team
from region import Region


def get_matches(client, tier: str, after: str, before: str) -> list:
    page = 1
    matches = []

    while True:
        current_page_matches = client.get_matches(tier=tier, mode='3', after=after, before=before, page=page)
        if not current_page_matches:
            break
        matches += current_page_matches
        page += 1

    return matches


def download_all_data():
    with Octane() as client:
        matches = []
        start_dates = [
            '2020-07-31', '2020-10-27', '2021-03-02', '2021-05-25', '2021-06-22', "2021-12-14", "2022-03-29",
            "2022-08-16", '2022-12-13'
        ]
        end_dates = [
            '2020-10-26', '2021-03-01', '2021-05-24', '2021-06-21', "2021-12-13", "2022-03-28", "2022-08-15",
            '2022-12-12', '2023-04-10'
        ]

        for start, end in zip(start_dates, end_dates):
            print("-----------------------------------------------------")
            print(start, " -- ", end)
            temp_matches = []
            temp_matches += get_matches(client, "S", start, end)
            temp_matches += get_matches(client, "A", start, end)
            temp_matches.reverse()
            matches += temp_matches

    # Export matches to JSON
    with open('data/matches.json', 'w') as f:
        json.dump(matches, f, indent=4)


def sort_matches():
    with open('data/matches.json', 'r') as f:
        matches = json.load(f)

    matches.sort(key=lambda x: x["date"])

    with open('data/matches.json', 'w') as f:
        json.dump(matches, f, indent=4)


if __name__ == '__main__':
    download_all_data()
    sort_matches()
