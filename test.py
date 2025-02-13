# This file is included as a test client for the server. 

import requests
import json


# Testing for part 1
def part1():
    username = 'thy'
    password = 'pswd'
    url = 'https://google.com'

    d = {'username': username, 'password': password, 'url': url}

    headers = {"Content-Type": "application/json"}
    body = json.dumps(d)
    # print(body)

    url = "http://localhost:3000/api/jamf/credentials"

    response = requests.post(url,
        body, headers= headers)

    print(response.text)

# Testing for part 2: 

# Testing API get call
def part2():
    url = "http://localhost:3000/api/jamf/devices"

    response = requests.get(url)
    print(response.text)

# Testing authourization 
def accessToken():
    username = 'thyland'
    password = 'Tomh101502!'
    url = "https://zipziptest.jamfcloud.com/api/v1/auth/token"

    response = requests.post(url, auth= (username, password))
    print(response.text)

part1()
part2()