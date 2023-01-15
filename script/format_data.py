import json

file_path = 'output/result.json'
sorted_file_path = 'output/sorted_result.json'
with open(file_path, 'r') as f:
    data = json.load(f)
    sorted_data = sorted(data.items(), key=lambda x: x[1], reverse=True)
    with open(sorted_file_path, 'w') as f:
        json.dump(sorted_data, f, indent=2, ensure_ascii=False)
