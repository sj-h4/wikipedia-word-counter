import json
import sys

args = sys.argv
file_path = args[1]
sorted_file_path = args[2]
with open(file_path, 'r') as f:
    data = json.load(f)
    sorted_data_list = sorted(data.items(), key=lambda x: x[1], reverse=True)
    sorted_data = dict((k, v) for k, v in sorted_data_list)
    with open(sorted_file_path, 'w') as f:
        json.dump(sorted_data, f, indent=2, ensure_ascii=False)
