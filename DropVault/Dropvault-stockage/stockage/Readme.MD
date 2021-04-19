 
 
 Upload
 curl -v -H "Content-Type: application/json" -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Im1vMjhPYXAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJyb21haW5sIiwiY29tcGFueSI6IkRyb3BWYXVsdCIsImV4cCI6MTYxNDc4Njg4OH0.jOKHlRAsvOeQkYh6LbSxT8Z1CahNJyRVWqQJs-hAj70' -X POST -d '{"filename": "file.txt", "username": "michel", "sizing": "15", "extension": "image/"}' 127.0.0.1:8084/files
 
 get files
 curl -X GET http://localhost:8084/files/michel

curl -v -X GET -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Ik12dzJybGEiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJtaWNoZWwiLCJjb21wYW55IjoiRHJvcFZhdWx0IiwiZXhwIjoxNjE0MTk2Nzk5fQ.MxIHwqxgQ_5UQ4dOOT0wgjpbGJCJ-saWVJfMT-cqVbU'   -i 'http://127.0.0.1:8084/files/michel'

User
curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "passwd": "mich2310"}' 127.0.0.1:8080/signup

curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel",  "passwd": "mich2310"}' 127.0.0.1:8080/login


curl -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6IldadXcyaXUiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJyb21haW5sIiwiY29tcGFueSI6IkRyb3BWYXVsdCIsImV4cCI6MTYxNDg3MzUwMH0.sfvME1jythE_C2HL4q8N59jKTKFbfXDBzVrlzddC-hs' -X POST -d '{"content": "blahblahblah"}' http://localhost:8084/upload

curl -v -X POST -H "Content-Type: application/json" --data-raw '{"content":"61433bd1ae50a17aa", "key":"maclef"}' http://localhost:8084/upload/151


