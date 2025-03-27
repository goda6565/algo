fn bubble_sort(target: &mut [i32]) {
    // バブルソートO(n^2)
    let path = target.len() - 1;
    for i in 0..path {
        for j in 0..path - i {
            if target[j] > target[j + 1] {
                target.swap(j, j + 1);
            }
        }
    }
}

fn insertion_sort(target: &mut [i32]) {
    // 挿入ソート
    for i in 1..target.len() {
        let key = target[i];
        let mut j = i as isize - 1; // iはusizeなのでisizeに変換
        while j >= 0 && target[j as usize] > key {
            // >= はusizeとisizeの比較でエラーになる（0はisize） リスト参照はusizeで行う
            target[j as usize + 1] = target[j as usize];
            j -= 1;
        }
        target[(j + 1) as usize] = key
    }
}

fn merge_sort(target: &mut [i32]) {
    // マージソート
    if target.len() <= 1 {
        return; // 要素が1つ以下なら何もしない
    }

    let mid = target.len() / 2;

    // 分割して再帰的にソート
    {
        let (left, right) = target.split_at_mut(mid);
        merge_sort(left);
        merge_sort(right);
    }

    // マージするための一時バッファを作成
    let mut sorted = target.to_vec();

    // マージ処理
    let mut i = 0; // left側のインデックス
    let mut j = mid; // right側のインデックス
    let mut k = 0; // sorted配列のインデックス

    // 左右の配列を比較しながらマージ
    while i < mid && j < target.len() {
        if target[i] <= target[j] {
            sorted[k] = target[i];
            i += 1;
        } else {
            sorted[k] = target[j];
            j += 1;
        }
        k += 1;
    }

    // 左側の残りの要素をコピー
    while i < mid {
        sorted[k] = target[i];
        i += 1;
        k += 1;
    }

    // 右側の残りの要素をコピー
    while j < target.len() {
        sorted[k] = target[j];
        j += 1;
        k += 1;
    }

    // ソート結果を元の配列にコピー
    target.copy_from_slice(&sorted);
}

fn shell_sort(target: &mut [i32]) {
    let mut distance = target.len();
    while distance > 0 {
        for i in distance..target.len() {
            let temp = target[i];
            let mut j = i;
            while j >= distance && target[j - distance] > temp {
                target[j] = target[j - distance];
                j -= distance;
            }
            target[j] = temp;
        }
        distance = distance / 2
    }
}

fn selection_sort(target: &mut [i32]) {
    for i in (1..target.len()).rev() {
        let mut max_index = 0;
        for j in 1..i + 1 {
            if target[j] > target[max_index] {
                max_index = j
            }
        }
        target.swap(i, max_index);
    }
}

fn main() {
    let mut arr = [5, 3, 8, 6, 7, 2];
    bubble_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr2 = [5, 3, 8, 6, 7, 2];
    insertion_sort(&mut arr2);
    println!("{:?}", arr2);

    let mut arr3 = [5, 3, 8, 6, 7, 2];
    merge_sort(&mut arr3);
    println!("{:?}", arr3);

    let mut arr4 = [5, 3, 8, 6, 7, 2];
    shell_sort(&mut arr4);
    println!("{:?}", arr4);

    let mut arr5 = [5, 3, 8, 6, 7, 2];
    selection_sort(&mut arr5);
    println!("{:?}", arr5);
}
