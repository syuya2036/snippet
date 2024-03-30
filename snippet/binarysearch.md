# 二分探索

```python
def solve(x):
	return True

left = 0
right = 1e10
while right - left > 1:
	mid = (left + right) // 2
	if solve(mid):
		right = mid
	else:
		left = mid
print(right)
```
## bisect

bisect_left(A, i)で，Aのソート状態を崩さずにiを挿入することができるインデックスのうち最も左のもの．
[1, 2 , 3, 3, 4],3を渡すと2が返ってくる．
