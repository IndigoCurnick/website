# Website

```
docker build -t indigowebsite .
heroku container:login
heroku container:push web -a indigowebsite
heroku container:release web -a indigowebsite
```