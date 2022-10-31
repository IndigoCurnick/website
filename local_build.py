import os


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


def main():

    chosen = False

    while not chosen:
        print("Welcome to the website builder!")
        print("Please select your desired build style")
        print("1 - Local with docker-compose")
        print("2 - Local with kubernetes (not supported yet)")
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
