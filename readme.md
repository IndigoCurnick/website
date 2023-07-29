```
sudo systemctl start docker
systemctl daemon-reload
systemctl enable cri-docker.service
systemctl enable --now cri-docker.socket
kubeadm init
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

## Instructions for setting up dev env

These steps are for an Arch-based distro

You first need to install the following things

```
sudo pacman -S python-pipenv postgresql minikube docker
```

## Instructions for setting up production environment

## How to build docker image

```
DOCKER_BUILDKIT=1 docker image build -t website .
```

## How to push docker images

```
docker tag website nathanielcurnick/website:dev
docker push nathanielcurnick/website:dev
```

## Private Docker Images

```
kubectl create secret generic regcred \
    --from-file=.dockerconfigjson=<path/to/.docker/config.json> \
    --type=kubernetes.io/dockerconfigjson
```