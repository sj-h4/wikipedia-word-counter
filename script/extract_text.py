import json
import sys

args = sys.argv
file_path = args[1]
write_file_path = args[2]
with open(file_path, 'r') as f:
    for line in f:
        data = json.loads(line)
        text = data['text']

        with open(write_file_path, 'a') as f:
            f.write(text)
