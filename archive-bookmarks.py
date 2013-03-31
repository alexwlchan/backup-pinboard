#!/usr/bin/python

import os
import pytz
from datetime import datetime
import urllib2

# Parameters.
bookmarkdir = os.environ['HOME'] + '/Dropbox/Personal/pinboard/'
pinboard_api = 'https://api.pinboard.in/v1/'
yearfmt = '%Y'
datefmt = '%m-%d'
homeTZ = pytz.timezone('GMT')
y = datetime.now(pytz.utc).strftime(yearfmt)
t = datetime.now(pytz.utc).strftime(datefmt)

# Get the user's authentication token
with open(os.environ['HOME'] + '/.pinboard-credentials') as credentials:
	for line in credentials:
		me, token = line.split(':')

if not os.path.exists(bookmarkdir + y):
	os.makedirs(bookmarkdir + y)

# Set up a new bookmarks file
bookmarkfile = open(bookmarkdir + y + '/pinboard-backup.' + t + '.xml', 'w')

# Get all the posts from Pinboard
u = urllib2.urlopen(pinboard_api + 'posts/all?auth_token=' + me + ':' + token)
bookmarkfile.write(u.read())
bookmarkfile.close()