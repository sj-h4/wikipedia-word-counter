import json

data_2006 = dict()
data_2023 = dict()

# 上位5000件のみを抽出
with open('output/2006/sorted_result.json') as f:
    all_data = json.load(f)
    data_2006 = {k: all_data[k] for k in list(all_data)[:5000]}

with open('output/2023/sorted_result.json') as f:
    all_data = json.load(f)
    data_2023 = {k: all_data[k] for k in list(all_data)[:5000]}

# 2006年と2023年の比が小さい順に並べる
# 2006年のデータが2023年に存在しない場合は、そのデータを出力する
ratio = dict()
absent = dict()
for k, v in data_2006.items():
    value_2023 = data_2023.get(k)
    if value_2023 is None:
        absent[k] = v
        continue
    ratio[k] = value_2023 / v

print('absent', absent)
with open('output/compare/ratio.json', 'w') as f:
    sorted_ratio_list = sorted(ratio.items(), key=lambda x: x[1])
    sorted_ratio = dict((k, v) for k, v in sorted_ratio_list)
    json.dump(sorted_ratio, f, indent=2, ensure_ascii=False)
