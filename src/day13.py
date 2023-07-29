from collections import deque

def parse_input(filename: str) -> tuple:
    packet_lines = list()
    with open(filename, 'r') as my_file:
        my_input = my_file.read()
    my_input = my_input.split()

    for each in my_input:
        # remove empty lines
        if len(each) == 0:
            continue
        packet_lines.append(each)
    return packet_lines


def parse_packet(raw_packet: str) -> list:
    return eval(raw_packet)


def trinary_compare(left: int, right: int) -> int:
    if left < right:
        return 1
    elif left > right:
        return -1
    return 0


def recursive_compare(packet_left: list, packet_right: list) -> int:
    # compare values
    if type(packet_left) == type(packet_right) and type(packet_left) == type(1):
        return trinary_compare(packet_left, packet_right)

    # one of them isn't a list, so list-ify it
    if type(packet_left) != type([]):
        packet_left = [packet_left]
    if type(packet_right) != type([]):
        packet_right = [packet_right]

    # compare lists
    for index in range(min(len(packet_left), len(packet_right))):
        my_result = recursive_compare(packet_left[index], packet_right[index])
        if my_result != 0:
            return my_result

    # out of order if left has more items than right
    return trinary_compare(len(packet_left), len(packet_right))


def sum_ordered_packets(packets: list) -> int:
    packet_sum = 0

    for each_index in range(int(len(packets)/2)):
        packet_left = parse_packet(packets[each_index*2])
        packet_right = parse_packet(packets[(each_index*2)+1])
        # check if packets[each_index] and packets[each_index+1] are correctly ordered
        if recursive_compare(packet_left=packet_left, packet_right=packet_right) == 1:
            # print(f"{str(packet_left)} and {str(packet_right)} are correctly ordered")
            packet_sum += (each_index + 1)
        else:
            # print(f"{str(packet_left)} and {str(packet_right)} are NOT correctly ordered")
            pass

    return packet_sum


def get_decoder_key(packets: list) -> int:
    packets = [parse_packet(x) for x in packets]
    # add the divider packets
    packets.append([[2]])
    packets.append([[6]])

    # we're going to do a bubble sort because why not
    is_sorted = False
    while not is_sorted:
        swapped = False
        for index in range(len(packets)-1):
            if recursive_compare(packets[index], packets[index+1]) == -1:
                # print(f"Swapping {packets[index]} and {packets[index+1]}")
                temp = packets[index]
                packets[index] = packets[index+1]
                packets[index+1] = temp
                swapped = True
                break
        if not swapped:
            break

    # multiply the indices of the two divider packets
    print(f"Found [[2]] at {packets.index([[2]]) + 1}")
    print(f"Found [[6]] at {packets.index([[6]]) + 1}")
    return (packets.index([[2]]) + 1) * (packets.index([[6]]) + 1)


def main():
    packets = parse_input('day13_input.txt')
    packets_sum = sum_ordered_packets(packets)
    print(f"The sum of correctly ordered packet indices is {packets_sum}")
    print(f"The decoder key is {get_decoder_key(packets)}")


if __name__ == '__main__':
    main()
