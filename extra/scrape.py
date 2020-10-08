#!/usr/bin/env fades

"""
scrape.py - A script to upload random cocktails.

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


def test_ingredients():
    raw = requests.get('https://www.thecocktaildb.com/api/json/v1/1/random.php').json()['drinks'][0]
    print('Cocktail:', raw['idDrink'])
    for i in count(1):
        if raw.get(f'strMeasure{i}') is not None:
            parse_measure(raw.get(f'strMeasure{i}'))
        else:
            break


def test():
    #raw = requests.get('https://www.thecocktaildb.com/api/json/v1/1/random.php').json()
    pprint(convert_json({'drinks': [{'dateModified': '2017-04-24 22:18:22',
             'idDrink': '15182',
             'strAlcoholic': 'Alcoholic',
             'strCategory': 'Ordinary Drink',
             'strCreativeCommonsConfirmed': 'No',
             'strDrink': 'After sex',
             'strDrinkAlternate': None,
             'strDrinkDE': None,
             'strDrinkES': None,
             'strDrinkFR': None,
             'strDrinkThumb': 'https://www.thecocktaildb.com/images/media/drink/xrl66i1493068702.jpg',
             'strDrinkZH-HANS': None,
             'strDrinkZH-HANT': None,
             'strGlass': 'Highball glass',
             'strIBA': None,
             'strIngredient1': 'Vodka',
             'strIngredient10': None,
             'strIngredient11': None,
             'strIngredient12': None,
             'strIngredient13': None,
             'strIngredient14': None,
             'strIngredient15': None,
             'strIngredient2': 'Creme de Banane',
             'strIngredient3': 'Orange juice',
             'strIngredient4': None,
             'strIngredient5': None,
             'strIngredient6': None,
             'strIngredient7': None,
             'strIngredient8': None,
             'strIngredient9': None,
             'strInstructions': 'Pour the vodka and creme over some ice cubes '
                                'in a tall glass and fill up with juice. to '
                                'make it beuty full make the top of the glass '
                                'with a grenadine and sugar',
             'strInstructionsDE': 'Gießen Sie den Wodka und die Sahne über '
                                  'einige Eiswürfel in ein hohes Glas und '
                                  'füllen Sie ihn mit Saft. Damit es voll ist, '
                                  'verzieren Sie die Oberseite des Glases mit '
                                  'einer Grenadine und Zucker.',
             'strInstructionsES': None,
             'strInstructionsFR': None,
             'strInstructionsZH-HANS': None,
             'strInstructionsZH-HANT': None,
             'strMeasure1': '2 cl ',
             'strMeasure10': None,
             'strMeasure11': None,
             'strMeasure12': None,
             'strMeasure13': None,
             'strMeasure14': None,
             'strMeasure15': None,
             'strMeasure2': '1 cl ',
             'strMeasure3': None,
             'strMeasure4': None,
             'strMeasure5': None,
             'strMeasure6': None,
             'strMeasure7': None,
             'strMeasure8': None,
             'strMeasure9': None,
             'strTags': None,
             'strVideo': None}]}))

if __name__ == '__main__':
    main()