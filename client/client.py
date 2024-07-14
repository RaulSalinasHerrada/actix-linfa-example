import json
import os
import requests

if __name__ == '__main__':
    url = "http://127.0.0.1:8000/predict_tree"
    this_dir = os.path.dirname(os.path.abspath(__file__))
    record_path = os.path.join(this_dir, 'record.json')

    print(f"Reading image from {record_path}")
    with open(record_path, 'r') as file:
        records = json.load(file)

    data = {
        'records': records
    }
    
    print(f"POST to {url}")
    
    res = requests.post(url,
                        json=data,
                        headers={'content-type': 'application/json'})
    print(f"Response ({res.status_code})")
    if res.status_code == 200:
        print(f"Content: {json.dumps(res.json(), indent=4)}")
    else:
        print(f"Body: {res.text}")