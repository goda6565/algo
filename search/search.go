package main

import "fmt"

func LinearSearch(targetList []int, target int) bool {
	for _, i := range targetList {
		if target == i {
			return true
		}
	}
	return false
}

func BinarySearch(targetList []int, target int) bool {
	first := 0
	last := len(targetList) - 1
	for first <= last {
		mid := (first + last) / 2
		if target == targetList[mid] {
			return true
		}
		if target > targetList[mid] {
			first = mid + 1
		} else {
			last = mid - 1
		}
	}
	return false
}

func InterpolationSearch(targetList []int, target int) bool {
	first := 0
	last := len(targetList) - 1
	for first <= last {
		mid := first + ((target - targetList[first]) * (last - first) / (targetList[last] - targetList[first]))
		if target == targetList[mid] {
			return true
		}
		if target > targetList[mid] {
			first = mid + 1
		} else {
			last = mid - 1
		}
	}
	return false
}

func main() {
	list := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	fmt.Println(LinearSearch(list, 5))
	fmt.Println(BinarySearch(list, 5))
	fmt.Println(InterpolationSearch(list, 5))
}
