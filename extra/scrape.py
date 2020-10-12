#!/usr/bin/env fades

"""
scrape.py - A script to upload random cocktails to a MongoDB database.

Requests 100 random cocktails from
https://www.thecocktaildb.com/api/json/v1/1/random.php
and upload them into a MongoDB database.

The MongoDB database is configured with the MDB_URL env var.

This script is designed to be run with `fades <https://github.com/PyAr/fades>`_,
which will handle installing the required dependencies into a Python virtualenv for you.
"""

from itertools import count
import os
from pprint import pprint
import re
import sys
import time

import requests # fades
import pymongo  # fades pymongo[srv]


def convert_json(raw):
    our_structure = {
        'name': raw['strDrink'],
        'instructions': raw['strInstructions'].split('. '),
        'ingredients': []
    }

    for i in count(1):
        if raw.get(f'strIngredient{i}') is not None:
            ingredient = {
                'name': raw[f'strIngredient{i}'],
            }
            if raw[f'strMeasure{i}'] is not None:
                ingredient['quantity'] = parse_measure(raw[f'strMeasure{i}'])

            our_structure['ingredients'].append(ingredient)
        else:
            break

    return our_structure


def parse_measure(measure_str):
    """
    Attempt to convert the measurement strings used by CocktailDB to the
    structure used by my database.
    """

    if measure_str is None:
        return None
    measure_str = measure_str.strip()
    if match := re.match(r'(?:Juice of )?(?P<quantity>[\d/]*\d(?: [\d/.]*\d)?)(?: (?P<unit>ozs?|parts?|mls?|cls?|shots?|tsps?|tbl?sps?|cups?|pinch(?:es)?|splash(?:es)?|dash(?:es)?|scoops?|drops?))?$', measure_str):
        result = {
            'quantity': match.group('quantity'),
            'unit': match.group('unit'),
        }
        print(repr(measure_str), result)
        return result
    else:
        raise Exception("Can't parse " + repr(measure_str))


def main():
    client = pymongo.MongoClient(os.environ['MDB_URL'])
    recipes = client.cocktails.recipes

    for _ in range(100):
        try:
            raw = requests.get('https://www.thecocktaildb.com/api/json/v1/1/random.php').json()['drinks'][0]
            print('Cocktail:', raw['idDrink'])
            upload = convert_json(raw)
            pprint(upload)
            recipes.insert_one(upload)
        except Exception as e:
            print(e, file=sys.stderr)
        time.sleep(1)


if __name__ == '__main__':
    main()