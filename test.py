import requests

HOST = "localhost"
PORT = "8080"

def test_create_new_job():

    new_job = {
        "name":"test_1",
        "job_type":"autopsy",
        "arguments":None,
        "file":None
    }

    response = requests.post(
        "http://"+HOST+":"+PORT+"/job/new",
        json=new_job
    )

    assert(response.ok)

def test_fail_to_create_new_job():

    new_job = {
        "name":"test_2",
        "job_type":"autopsy",
        "arguments":None,
        "file":None,
        "extra_field":1337
    }

    response = requests.post(
        "http://"+HOST+":"+PORT+"/job/new",
        json=new_job
    )

    assert(response.ok == False)

    new_job = {
        "name":"test_3",
        "job_type":"autopsy",
    }

    response = requests.post(
        "http://"+HOST+":"+PORT+"/job/new",
        json=new_job
    )

    assert(response.ok == False)
