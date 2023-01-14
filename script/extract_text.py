import json

file_path = 'output/wiki.json'
with open(file_path, 'r') as f:
    for line in f:
        data = json.loads(line)
        text = data['text']

        with open('output/wiki.txt', 'a') as f:
            f.write(text)
