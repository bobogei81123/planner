import json
import requests

response = requests.post('http://localhost:8000/auth/login', json={
    'username': 'meteor',
})
response_json = response.json()
token = response_json['token']
print(json.dumps({'Authorization': f'Bearer {token}'}))
