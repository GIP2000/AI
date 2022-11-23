import matplotlib.pyplot as plt
def main():

    X1 = []
    Y1 = []
    X2 = []
    Y2 = []

    with open("./spiral_train.txt") as f:
        for (i,line) in enumerate(f):
            if i == 0:
                continue
            data = line.split(" ")
            if int(data[2]) == 0:
                X1.append(float(data[0]))
                Y1.append(float(data[1]))
            else:
                X2.append(float(data[0]))
                Y2.append(float(data[1]))


    plt.scatter(X1,Y1, color="red")
    plt.scatter(X2,Y2, color="blue")
    plt.show()



if __name__ == "__main__":
    main()



