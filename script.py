import os, re

pattern = re.compile("Fig\d{4}\(.\)\((.*)\)\.tif")

for f in os.listdir('./images'):
    match = pattern.findall(f)
    if match != []:
        os.rename("./images/{}".format(f), "./images/{}.tif".format(match[0]))
