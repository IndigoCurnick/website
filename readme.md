```
sudo systemctl start docker
systemctl daemon-reload
systemctl enable cri-docker.service
systemctl enable --now cri-docker.socket
kubdadm init
kubectl taint nodes --all node-role.kubernetes.io/control-plane-
kubectl apply -f calico.yaml
```


[Postgres Instructions](https://phoenixnap.com/kb/postgresql-kubernetes)

Create postgres config map
`kubectl apply -f postgres-configmap.yaml`

Then create the persistent volume
`kubectl apply -f postgres-storage.yaml`

The create the deployment and service
`kubectl apply -f postgres-deployment.yaml`

From ~ run

```
source pgadmin4/bin/activate
pgadmin4
```

To get pg amdin running

Then log into database and run the sql file inside

[ ] Have the sql file automatically load

Then apply website deployment/service with
`kubectl apply -f website.yaml`

Then the website will be hosted at
`minikube ip`

With port 30100

To clean up use
`kubectl delete all --all`

Ingress
`minikube addons enable ingress`

You can add hosts in /etc/hosts to simulate the ingress on the internet
