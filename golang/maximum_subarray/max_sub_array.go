package maximumsubarray

func MaxSubArray(nums []int) int {
	var max, maxHere int

	max = nums[0]
	maxHere = nums[0]

	for i := 1; i < len(nums); i++ {
		maxHere = maxInt(maxHere+nums[i], nums[i])
		max = maxInt(max, maxHere)
	}

	return max
}

func maxInt(x, y int) int {
	if x > y {
		return x
	}

	return y
}
