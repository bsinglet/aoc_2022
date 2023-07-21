from collections import deque

def parse_input(filename: str) -> tuple:
    height_map = list()
    starting_location = (-1, -1)
    ending_location = (-1, -1)
    with open(filename, 'r') as my_file:
        my_input = my_file.read()
    my_input = my_input.split()

    for y in range(len(my_input)):
        if 'S' in my_input[y]:
            starting_location = (my_input[y].find('S'), y)
        if 'E' in my_input[y]:
            ending_location = (my_input[y].find('E'), y)
        height_map.append(my_input[y])
        height_map[y] = [x for x in height_map[y]]
    return starting_location, ending_location, height_map


def get_neighbors(this_node, height_map) -> list:
    this_node_height = height_map[this_node[1]][this_node[0]]
    if this_node_height == 'S':
        this_node_height = 'a'
    elif this_node_height == 'E':
        this_node_height = 'z'
    
    candidates = list()
    neighbors = list()
    # only check neighbors that are actually on the map
    if this_node[0] - 1 >= 0:
        candidates.append((this_node[0]-1, this_node[1]))
    if this_node[0] + 1 < len(height_map[this_node[1]]):
        candidates.append((this_node[0]+1, this_node[1]))
    if this_node[1] - 1 >= 0:
        candidates.append((this_node[0], this_node[1]-1))
    if this_node[1] + 1 < len(height_map):
        candidates.append((this_node[0], this_node[1]+1))
    
    # make sure the neighbor is below or at most one heigh above the current location
    for each in candidates:
        target_height = height_map[each[1]][each[0]]
        if target_height == 'S':
            target_height = 'z'
        elif target_height == 'E':
            target_height = 'z'
        if ord(this_node_height) >= ord(target_height) - 1:
            neighbors.append(each)
    
    return neighbors


def breadth_first_search(starting_location, ending_location, height_map) -> int:
    short_path_length = 0
    distance_table = dict()
    visited_nodes = list()

    # initialize our distance table
    for y in range(len(height_map)):
        for x in range(len(height_map[y])):
            if height_map[y][x] == 'S':
                distance_table[(x, y)] = {'Distance': 0, 'Previous vertex': None}
            else:
                distance_table[(x, y)] = {'Distance': -1, 'Previous vertex': None}

    # initialize our node queue
    node_queue = deque()
    next_layer_node_queue = deque()
    node_queue.append(starting_location)

    found = False
    while len(node_queue) > 0 and not found:
        this_node = node_queue.popleft()
        visited_nodes.append(this_node)
        for each_neighbor in get_neighbors(this_node, height_map):
            # update distance in distance table
            if distance_table[(each_neighbor[0], each_neighbor[1])]['Distance'] > short_path_length or distance_table[(each_neighbor[0], each_neighbor[1])]['Distance'] == -1:
                distance_table[(each_neighbor[0], each_neighbor[1])]['Distance'] = short_path_length
                distance_table[(each_neighbor[0], each_neighbor[1])]['Previous vertex'] = (this_node[0], this_node[1])
            # add neighbors to next_layer_node_queue if they're unvisited
            if each_neighbor not in visited_nodes and each_neighbor not in next_layer_node_queue:
                next_layer_node_queue.append((each_neighbor[0], each_neighbor[1]))
            # if this neighbor is the end_goal, break out of the while loop
            if height_map[each_neighbor[1]][each_neighbor[0]] == 'E':
                short_path_length += 1
                found = True
                break

        # go to next depth if necessary
        if len(node_queue) == 0 and not found:
            short_path_length += 1
            node_queue = next_layer_node_queue
            next_layer_node_queue = deque()
    
    return short_path_length


def main():
    starting_location, ending_location, height_map = parse_input('day12_input.txt')
    shorted_path_length = breadth_first_search(starting_location, ending_location, height_map)
    print(f"Shortest path is {shorted_path_length}")


if __name__ == '__main__':
    main()
