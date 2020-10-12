#!/usr/bin/env fades

"""
sample_records.py - Print out the structure of some sample records from the cocktails database.

The MongoDB database is configured with the MDB_URL env var.

This script is designed to be run with `fades <https://github.com/PyAr/fades>`_,
which will handle installing the required dependencies into a Python virtualenv for you.
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


def main():
    client = pymongo.MongoClient(os.environ['MDB_URL'])
    recipes = client.cocktails.recipes
    reviews = client.cocktails.reviews
    recipes_with_reviews = client.cocktails.recipes_with_reviews

    recipe = recipes.find_one({"name": "Negroni Sbagliato"})
    review = reviews.find_one({ "recipe_id": recipe['_id'] })

    with_reviews = recipes_with_reviews.find_one({"name": "Negroni Sbagliato"})

    pprint(recipe)
    pprint(review)

    pprint(with_reviews)


if __name__ == '__main__':
    main()