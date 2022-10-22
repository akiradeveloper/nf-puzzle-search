# nf-puzzle-search

京都大学は日本最高の大学である。
日本中の賢い学生が集まる大学、それが京都大学である。

京都大学の文化祭をNFという。
彼らは今年、面白いパズルを思いついた。
NFまでの残り日数をこんな感じで、
数字を固定したまま
演算子だけを変更することによって表そうしたのだ。
これはなかなか面白い試みだ。さすが京都大学。おれの母校だ。

![スクリーンショット 2022-10-22 17 27 31](https://user-images.githubusercontent.com/785824/197329435-733d1312-296d-4a49-8a1d-997562edc9dd.png)

ふと、疑問がよぎる。果たして、この数字の並びは最善だろうか？
0日からN日を表すことが出来る数字の並びのうち、この`[6,4,5,2,1]`という並びはもっとも長いNを持っているだろうか？

確かめるしかない。

## 確かめた。

NFが使ってる`[6,4,5,2,1]`という並びで作れる最大の数Nは35である。

では、これがN最大なのかというとそうではなく、
仮に5つの値がユニークでないことを許すならば
`[9,4,2,3,3]`はN=40である。

```
9-4÷2x3-3=0
9-4÷2-3-3=1
9x4÷2÷3÷3=2
9x4÷2÷3-3=3
9-4-2+3÷3=4
9+4÷2-3-3=5
9-4÷2x3+3=6
9-4÷2÷3x3=7
9x4x2÷3÷3=8
9x4÷2÷3+3=9
9-4x2+3x3=10
9+4÷2÷3x3=11
9x4÷2-3-3=12
9-4÷2+3+3=13
9-4+2x3+3=14
9+4+2÷3x3=15
9-4÷2+3x3=16
9x4÷2-3÷3=17
9x4÷2÷3x3=18
9x4÷2+3÷3=19
9+4÷2+3x3=20
9x4x2÷3-3=21
9+4+2x3+3=22
9-4+2x3x3=23
9x4÷2+3+3=24
9x4-2-3x3=25
9+4x2+3x3=26
9x4÷2+3x3=27
9x4-2-3-3=28
9x4+2-3x3=29
9+4x2x3-3=30
9+4+2x3x3=31
9x4+2-3-3=32
9x4-2x3+3=33
9x4-2÷3x3=34
9x4-2+3÷3=35
9+4x2x3+3=36
9x4+2-3÷3=37
9x4+2÷3x3=38
9x4+2x3-3=39
9x4-2+3+3=40
```

ユニークであることを制約にすると、`[9,3,5,6,1]`はN=3９である。

```
9-3x5+6÷1=0
9÷3+5-6-1=1
9÷3+5-6÷1=2
9÷3-5+6-1=3
9÷3-5+6÷1=4
9÷3-5+6+1=5
9-3-5+6-1=6
9-3-5+6÷1=7
9÷3x5-6-1=8
9÷3x5-6÷1=9
9÷3x5-6+1=10
9+3+5-6÷1=11
9+3-5+6-1=12
9÷3+5+6-1=13
9÷3+5+6÷1=14
9÷3+5+6+1=15
9x3-5-6÷1=16
9x3-5-6+1=17
9-3+5+6+1=18
9+3x5-6+1=19
9÷3x5+6-1=20
9÷3x5+6÷1=21
9÷3x5+6+1=22
9+3+5+6÷1=23
9+3+5+6+1=24
9x3+5-6-1=25
9x3+5-6÷1=26
9x3-5+6-1=27
9x3-5+6÷1=28
9x3-5+6+1=29
9+3x5+6÷1=30
9+3x5+6+1=31
9÷3+5x6-1=32
9÷3+5x6÷1=33
9÷3+5x6+1=34
9-3+5x6-1=35
9-3+5x6÷1=36
9x3+5+6-1=37
9x3+5+6÷1=38
9x3+5+6+1=39
```

## 実行方法

`cargo run --release search`
