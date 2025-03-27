package main

import (
	"fmt"
)

func BubbleSort(target []int) []int {
	// バブルソートO(n^2)
	path := len(target) - 1
	for i := 0; i < path; i++ {
		for j := 0; j < path-i; j++ {
			if target[j] > target[j+1] {
				target[j], target[j+1] = target[j+1], target[j]
			}
		}
	}
	return target
}

func InsertionSort(target []int) []int {
	// 挿入ソート
	for i := 1; i < len(target); i++ {
		key := target[i]
		j := i - 1
		for j >= 0 && key < target[j] {
			target[j+1] = target[j]
			j--
		}
		target[j+1] = key
	}
	return target
}

func MergeSort(target []int) []int {
	// マージソート
	if len(target) <= 1 {
		return target
	} else {
		mid := len(target) / 2
		left := make([]int, mid)
		right := make([]int, len(target)-mid)
		copy(left, target[:mid])
		copy(right, target[mid:])

		left = MergeSort(left)
		right = MergeSort(right)

		i := 0 // left
		j := 0 // right
		k := 0 // target

		for i < len(left) && j < len(right) {
			if left[i] < right[j] {
				target[k] = left[i]
				i += 1
			} else {
				target[k] = right[j]
				j += 1
			}
			k += 1
		}
		for i < len(left) {
			target[k] = left[i]
			i += 1
			k += 1
		}
		for j < len(right) {
			target[k] = right[j]
			j += 1
			k += 1
		}
	}
	return target
}

func ShellSort(target []int) []int {
	// シェルソート
	distance := len(target) / 2
	for distance > 0 {
		for i := distance; i < len(target); i++ {
			temp := target[i]
			j := i
			for j >= distance && target[j-distance] > temp {
				target[j] = target[j-distance]
				j -= distance
			}
			target[j] = temp
		}
		distance = distance / 2
	}
	return target
}

func SelectionSort(target []int) []int {
	for i := len(target) - 1; i > 0; i-- {
		max_index := 0
		for j := 1; j <= i; j++ {
			if target[j] > target[max_index] {
				max_index = j
			}
		}
		target[i], target[max_index] = target[max_index], target[i]
	}
	return target
}
func main() {
	target := []int{5, 3, 8, 6, 7, 2, 5}
	fmt.Println(BubbleSort(target))
	fmt.Println(InsertionSort(target))
	fmt.Println(MergeSort(target))
	fmt.Println(ShellSort(target))
	fmt.Println(SelectionSort(target))
}
