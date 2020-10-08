#!/usr/bin/env fades

"""
review.py - A script to add random reviews to cocktail recipes.


"""

from datetime import datetime, timedelta
from itertools import count
import os
from pprint import pprint
from random import normalvariate, randint, uniform
import re
import sys
import time

import requests # fades
import pymongo  # fades pymongo[srv]



def gen_ratings():
    rating_count = randint(0, 20)
    mean = randint(1, 5)
    std_dev = uniform(0, 2)
    return [
        {
        'when': datetime.now() - timedelta(days=uniform(0, 365)),
        'rating': max(min(round(normalvariate(mean, std_dev)), 5), 0),
        } for _ in range(rating_count)
    ]


def main():
    client = pymongo.MongoClient(os.environ['MDB_URL'])
    recipes = client.cocktails.recipes
    reviews = client.cocktails.reviews

    for recipe in recipes.find():
        if ratings := gen_ratings():
            for rating in ratings:
                rating['recipe_id'] = recipe['_id']
            reviews.insert_many(ratings)


if __name__ == '__main__':
    main()