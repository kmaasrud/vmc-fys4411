import os


def read_csv(filename):
    SEPARATOR = ","
    def process_line(line):
        line = line.split(SEPARATOR)
        # Strip any trailing whitespace or newlines
        line = [l.rstrip() for l in line] 

        # Try evaluating each element to a Python datatype.
        # This means that e.g. 2+2 will be evaluted as 4, and not a string
        try:
            line = [eval(l) for l in line]
        # NameError means its a string, just store the plain value
        except NameError:
            pass
        
        return line

    dictionary = {}
    with open(filename) as f:
        # Read in first line as header and add as keywords in dictionary
        header = process_line(f.readline())
        dictionary = {h: [] for h in header}

        # Each proceeding line will be added to its respective header
        for line in f.readlines():
            line = process_line(line)
            for i, h in enumerate(header):
                dictionary[h].append(line[i])

    return dictionary


def find_cargo_root():
    root_path = os.getcwd()
    while True:
        # Cargo.toml located in path, return the path
        if os.path.isfile(os.path.join(root_path, "Cargo.toml")):
            return root_path
        # If the paths parent is equal to itself, we're in the root. Raise error
        elif os.path.dirname(root_path) == root_path:
            raise FileNotFoundError(f"Could not find a Cargo root")
        # Change path to its parent and loop again
        else:
            root_path = os.path.dirname(root_path)

if __name__ == "__main__":
    print(find_cargo_root())
