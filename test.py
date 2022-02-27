import json
import requests

x = requests.post("http://127.0.0.1:8080/job", data={"username":"jonah"} )
print(x.content)
