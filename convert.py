# A File to convert Kubernetes yaml files to their arm counterparts

def main():
    website()
    postgres()


def website():
    with open("website.yaml") as f:
        text = f.read()

    new_text = text.replace("nathanielcurnick/website:latest",
                            "nathanielcurnick/website:arm")

    with open("website-arm.yaml", "w") as f:
        f.write(new_text)


def postgres():
    with open("postgres-deployment.yaml") as f:
        text = f.read()

    new_text = text.replace("postgres:latest",
                            "arm64v8/postgres")

    with open("postgres-deployment-arm.yaml", "w") as f:
        f.write(new_text)


if __name__ == "__main__":
    main()
