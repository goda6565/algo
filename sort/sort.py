def bubble_sort(target: list[int]) -> list[int]:
    # バブルソートO(n^2)
    path = len(target) - 1
    for i in range(path):
        for j in range(path - i):
            if target[j] > target[j + 1]:
                target[j], target[j + 1] = target[j + 1], target[j]

    return target


def insertion_sort(target: list[int]) -> list[int]:
    # 挿入ソート平均O(n^2)
    for i in range(1, len(target)):
        key = target[i]
        j = i - 1
        while j >= 0 and key < target[j]:
            target[j + 1] = target[j]
            j -= 1
        target[j + 1] = key
    return target


def merge_sort(target: list[int]) -> list[int]:
    # マージソートO(nlogn)（データがソートされていても性能は変わらない。）大規模向き
    if len(target) <= 1:
        return target
    else:
        mid = len(target) // 2
        left = target[:mid]
        right = target[mid:]

        left = merge_sort(left)
        right = merge_sort(right)

        i = 0  # left
        j = 0  # right
        k = 0  # target
        while i < len(left) and j < len(right):
            if left[i] < right[j]:
                target[k] = left[i]
                i += 1
            else:
                target[k] = right[j]
                j += 1
            k += 1  # これは毎回インクリメントされる
        while i < len(left):  # ここで残りの要素を追加（これは片方しか走らない）
            target[k] = left[i]
            i += 1
            k += 1
        while j < len(right):  # ここで残りの要素を追加（これは片方しか走らない）
            target[k] = right[j]
            j += 1
            k += 1
    return target


def shell_sort(target: list[int]) -> list[int]:
    # シェルソート中規模向き
    distance = len(target) // 2
    while distance > 0:
        for i in range(distance, len(target)):
            temp = target[i]
            j = i
            # 現在の間隔（distance）ごとにグループ化されたサブリストに対して挿入ソート
            while j >= distance and target[j - distance] > temp:
                target[j] = target[j - distance]
                j -= distance
            target[j] = temp
        # 間隔を半分にする
        distance //= 2
    return target


def selection_sort(target: list[int]) -> list[int]:
    # 選択ソート
    for fill_slot in range(len(target) - 1, 0, -1):
        max_index = 0
        for location in range(1, fill_slot + 1):
            if target[location] > target[max_index]:
                max_index = location
        target[fill_slot], target[max_index] = target[max_index], target[fill_slot]
    return target


if __name__ == "__main__":
    target_list = [5, 3, 8, 6, 7, 2, 5, 1, 4, 9, 10, 0]
    print(bubble_sort(target_list))
    print(insertion_sort(target_list))
    print(merge_sort(target_list))
    print(shell_sort(target_list))
    print(selection_sort(target_list))
