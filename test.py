from time import sleep
import requests
import json

# Login
login_url = 'http://127.0.0.1:8000/api/login/'


login_data = {
    'username': 'useris',
    'password': 'labas',
}

headers = {
    'Content-Type': 'application/json',
}

try:

    login_response = requests.post(login_url, data=json.dumps(login_data), headers=headers)

    if login_response.status_code == 200:

        
                                      
        jwt_token = login_response.json().get('token')

        print(jwt_token+'\n')

                  
        authenticated_url = 'http://127.0.0.1:8000/api/authors?page=1&search=80'
                        #                  ^
                        #                  |
                        #                  |
                        # http://127.0.0.1:8000/api/posts      GET, POST
                        # http://127.0.0.1:8000/api/posts/1    GET, DELETE, PATCH
                        # http://127.0.0.1:8000/api/authors    GET, POST
                        # http://127.0.0.1:8000/api/authors/1  GET, DELETE, PATCH
                        # http://127.0.0.1:8000/api/posts/author/1      GET
    
       

        sleep(1)
        headers['Authorization'] = f'Bearer {jwt_token}'
        data = {
            "name": "John",
            "surname": "810"
        }
        
        data_json = json.dumps(data)
     
                                #post, get, patch, delete
        authenticated_response = requests.get(authenticated_url, headers=headers, data=data_json)
        print(authenticated_response.text)

        if authenticated_response.status_code == 200:

            print('Response JSON:', authenticated_response.json())
        else:
            print( authenticated_response.status_code)
   


except requests.exceptions.RequestException as e:
    print('Request error:', e)