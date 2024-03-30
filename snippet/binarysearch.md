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

