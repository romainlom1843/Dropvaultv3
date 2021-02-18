 
 
 Upload
 curl -v -H "Content-Type: application/json"  -X POST -d '{"filename": "file.txt", "content": "bar1", "username": "michel"}' 127.0.0.1:8084/files
 
 get files
 curl -X GET http://localhost:8084/files/michel

curl -v -X GET -H 'Authorization: bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImlCZmxxTEcwNEt5TFd5ZTRuc0FtMCJ9.eyJpc3MiOiJodHRwczovL2Rldi12cjVhY3BmeC5ldS5hdXRoMC5jb20vIiwic3ViIjoiaXpVZ2ZTbmh5UmxMZmxQVHhraHlUcUdNeWJEUkRCTlhAY2xpZW50cyIsImF1ZCI6Imh0dHBzOi8vZXhhbXBsZS5jb20iLCJpYXQiOjE2MTM2MzQzNzQsImV4cCI6MTYxMzcyMDc3NCwiYXpwIjoiaXpVZ2ZTbmh5UmxMZmxQVHhraHlUcUdNeWJEUkRCTlgiLCJndHkiOiJjbGllbnQtY3JlZGVudGlhbHMifQ.tTrwU3Z4LuJQZZHQiYHjZcUUS6PDfS5lDfQNGGQAmrbqEjbOWekKkpdvvr4s8cJAo0OC9bUgu4Vjz62n21mllwo79pRR4dtW_sOq3Kd7bVq_ZyRVSNKB4OWISJaNRdRVgoeR55rM3fhEubacuKxjPUqg4o9JbPJ6QHQ7yN_yLsYeAFYFSEBHuZYBvHcjFcprtpH7JykSk178jQwSSd5S1ydxRhJ0SfBNPAwjFiOVZFTc5NZQ36zOPe7P3b4nNPNyE8JrNFJNIdZyVKv4WKOpi8L5awDwKnF8_7oaKBy2xY_PEPFUcQA2dApw0zoSToRHN99HshpXKaSME7rBog'   -i 'http://127.0.0.1:8084/files/michel'

User
curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "password": "mich2310"}' 127.0.0.1:8080/signup
