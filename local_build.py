import os
import psycopg2


def local_docker_compose():

    chosen = False
    while not chosen:
        choice = input(
            "Would you like to rebuild the docker image of the website? y/n")

        match choice:
            case "y":
                # os.system("docker rmi website")
                os.system("DOCKER_BUILDKIT=1 docker build -t website .")
                chosen = True
            case "n":
                chosen = True
            case _:
                print("Input not recognised")

    os.system("docker-compose down")
    os.system("docker-compose build")
    os.system("docker-compose up")


def local_minikube():
    # TODO: Docker pushing would be awesome here
    print("Ensure that you have first pushed the right version to docker")

    os.system("minikube start")

    # Postgres
    os.system("kubectl apply -f postgres-configmap.yaml")
    os.system("kubectl apply -f postgres-storage.yaml")
    os.system("kubectl apply -f postgres-deployment.yaml")

    # TODO: Synchronise these with what's in the deployment files
    conn = psycopg2.connect(host="", database="prod",
                            user="postgress", password="")


def running_as_root() -> bool:
    return os.getuid() == 0


def main():

    RUNNING_AS_ROOT = running_as_root()

    if RUNNING_AS_ROOT:
        print("You are running as the root user")
        print("Therefore, I will attempt to start the necessary services")

    else:
        print("You are running as a non root user")
        print("I am unable to start the necessary services for you, therefore, please ensure Docker and other necessary services are running")

    chosen = False

    while not chosen:
        print("Welcome to the website builder!")
        print("Please select your desired build style")
        print("1 - Local with docker-compose")
        print("2 - Local with minikube (not supported yet)")
        print("3 - Local with kubeadm (not supported yet)")
        selection = input("Please enter your selection")

        match selection:
            case "1":
                chosen = True
                local_docker_compose()
            case "2":
                print("Sorry, this isn't supported _yet_")
            case _:
                print("Unknown input - please try again")


if __name__ == "__main__":
    main()
