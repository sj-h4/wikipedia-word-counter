import json

type_count = 0
token_count = 0
top_20_type = dict()
with open('output/2023/sorted_result.json') as f:
    data = json.load(f)
    for k, v in data.items():
        type_count += 1
        token_count += v
        if type_count <= 20:
            top_20_type[k] = v

with open('output/2023/analysis.json', 'w') as f:
    json.dump(top_20_type, f, indent=2, ensure_ascii=False)

print('type_count', type_count)
print('token_count', token_count)
