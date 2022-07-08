import os


def main():
    os.system("docker rmi website")
    os.system("DOCKER_BUILDKIT=1 docker build -t website .")
    print("Done building - Running")
    os.system("docker run -p 8080:8080 website")


if __name__ == "__main__":
    main()
