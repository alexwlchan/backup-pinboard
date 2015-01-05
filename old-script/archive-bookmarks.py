#!/usr/bin/env python
# -*- coding: utf-8 -*-

import argparse
import os
import requests
import sys
from datetime import datetime


# Parameters
parser = argparse.ArgumentParser()
parser.add_argument("-d", "--directory",
                    help="Directory for bookmark file (default ~)")
parser.add_argument("-f", "--format", help="Format for dump (xml or json)")
args = parser.parse_args()

if args.directory:
    bookmarkdir = args.directory
else:
    bookmarkdir = os.environ['HOME']

if args.format == 'json' or args.format == 'JSON':
    dumpformat = 'json'
else:
    dumpformat = 'xml'

pinboard_api = 'https://api.pinboard.in/v1/'
yearfmt = '%Y'
datefmt = '%m-%d'
y = datetime.utcnow().strftime(yearfmt)
t = datetime.utcnow().strftime(datefmt)

backup = os.path.join(bookmarkdir, y, 'pinboard-backup_' + t + '.' + dumpformat)

outdir = os.path.dirname(backup)

if not os.path.exists(outdir):
    try:
        os.makedirs(outdir)
    except OSError:
        raise Exception("Couldn't create a directory at %s" % outdir)

"""
Get the user's authentication token
It's available at https://pinboard.in/settings/password
Store it in your home dir, in a file named .pinboard-credentials
"""
try:
    with open(os.path.join(os.environ['HOME'],
              '.pinboard-credentials')) as credentials:
        payload = {"auth_token": credentials.readline().strip(),
                   "format": dumpformat}
except IOError:
    raise Exception("Couldn't get your credentials from %s" % credentials.name)

if not payload.get("auth_token"):
    raise Exception(
        "There was a problem with your pinboard credentials:\n\
They should be stored in the format 'pinboard_username:xxxxxxxxxxxxxxxxxxxx'")

# Get all the posts from Pinboard
req = requests.get(pinboard_api + 'posts/all', params=payload)
# raise an exception for a 4xx code
req.raise_for_status()
print("Authentication successful, trying to write backup.")

# write a new bookmarks file
try:
    with open(backup, 'w') as o:
        o.write(req.text.encode("utf-8"))
except IOError:
    raise Exception("Couldn't create new bookmarks file at %s" % outdir)

print("Done! Backed up bookmarks to %s" % o.name)
