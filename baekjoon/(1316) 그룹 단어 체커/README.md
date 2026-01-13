# [(1316) 그룹 단어 체커](https://www.acmicpc.net/problem/1316)

| Time Limit | Memory Limit |
| :--------: | :----------: |
|     2s     |    128 MB    |

## Question

그룹 단어란 단어에 존재하는 모든 문자에 대해서, 각 문자가 연속해서 나타나는 경우만을 말한다. 예를 들면, ccazzzzbb는 c, a, z, b가 모두 연속해서 나타나고, kin도 k, i, n이 연속해서 나타나기 때문에 그룹 단어이지만, aabbbccb는 b가 떨어져서 나타나기 때문에 그룹 단어가 아니다.

단어 N개를 입력으로 받아 그룹 단어의 개수를 출력하는 프로그램을 작성하시오.

## Input

첫째 줄에 단어의 개수 N이 들어온다. N은 100보다 작거나 같은 자연수이다. 둘째 줄부터 N개의 줄에 단어가 들어온다. 단어는 알파벳 소문자로만 되어있고 중복되지 않으며, 길이는 최대 100이다.

## Output

첫째 줄에 그룹 단어의 개수를 출력한다.

## Examples

```
[Input]
3
happy
new
year

[Ouput]
3
```

```
[Input]
4
aba
abab
abcabc
a

[Output]
1
```

```
[Input]
5
ab
aa
aca
ba
bb

[Output]
4
```

```
[Input]
2
yzyzy
zyzyz

[Output]
0
```

```
[Input]
1
z

[Output]
1
```

```
[Input]
9
aaa
aaazbz
babb
aazz
azbz
aabbaa
abacc
aba
zzaz

[Output]
2
```
