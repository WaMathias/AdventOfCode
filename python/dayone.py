def get_digits(line):
    digits = []
    for i in range(len(line)):
        if line[i].isdigit():
            digits.append(int(line[i]))
        else:
            for word, digit in [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]:
                if line[i:].startswith(word):
                    digits.append(digit)
                    break
    return digits


def calculate_calibration_value(line):
    digits = get_digits(line)
    return int(str(digits[0]) + str(digits[-1]))


def main():
    with open("/home/mathias/Documents/AdventOfCode/calibrations.txt", "r") as f:
        lines = f.readlines()

    total_calibration_value = 0
    for line in lines:
        total_calibration_value += calculate_calibration_value(line)

    print(total_calibration_value)


if __name__ == "__main__":
    main()
