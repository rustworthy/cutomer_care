/questions GET
```
curl --location 'localhost:7878/questions'
```

/questions POST
```
curl -X POST 'localhost:7878/questions' -H 'Content-Type: application/json' -d '{"id": "90473158-5b63-11ed-9b6a-0242ac120002", "title": "Urgent question", "content": "Customer care phone is always busy"}'
```

/question/:uid GET
```
curl --location 'localhost:7878/questions/90473158-5b63-11ed-9b6a-0242ac120002'
```

/questions/:uid PUT
```
curl -X PUT 'localhost:7878/questions/0123124' -H 'Content-Type: application/json' -d '{"id": "90473158-5b63-11ed-9b6a-0242ac120002", "title": "Urgent question (CLOSED)", "content": "Customer care phone is always busy."}' 
```

/question/:uid DELETE
```
curl -X DELETE 'localhost:7878/questions/90473158-5b63-11ed-9b6a-0242ac120002'
```

