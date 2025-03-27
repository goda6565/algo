def linear_search(target_list: list[int], target: int) -> bool:
    # 線形探索O(n)
    for i in target_list:
        if i == target:
            return True
    return False


def binary_search(target_list: list[int], target: int) -> bool:
    # 二分探索O(logn)（ソート済みのリストに対して）
    first = 0
    last = len(target_list) - 1
    found = False
    while first <= last and not found:
        mid = (first + last) // 2
        if target_list[mid] == target:
            found = True
        else:
            if target < target_list[mid]:
                last = mid - 1
            else:
                first = mid + 1
    return found


def interpolation_search(target_list: list[int], target: int) -> bool:
    # 補間探索はソート済みリストが均一に分布している場合に高速です
    first = 0
    last = len(target_list) - 1
    found = False
    while (
        first <= last and target >= target_list[first] and target <= target_list[last]
    ):
        if target_list[first] == target_list[last]:  # リストに要素が一つしかない場合
            if target_list[first] == target:
                found = True
            return found

        # mid を整数計算で求める
        mid = first + ((last - first) * (target - target_list[first])) // (
            target_list[last] - target_list[first]
        )

        if target_list[mid] == target:
            found = True
            return found
        elif target < target_list[mid]:
            last = mid - 1
        else:
            first = mid + 1
    return found


if __name__ == "__main__":
    target_list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    target = 5
    print(linear_search(target_list, target))
    print(binary_search(target_list, target))
    print(interpolation_search(target_list, target))
    target = 11
    print(linear_search(target_list, target))
    print(binary_search(target_list, target))
    print(interpolation_search(target_list, target))
    target_list = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    print(interpolation_search(target_list, 1))
