# 1. Two Sum
กำหนดให้ nums เป็นอาร์เรย์ของจำนวนเต็ม และ target เป็นจำนวนเต็มหนึ่งจำนวน จงหาดัชนีของตัวเลขสองตัวในอาร์เรย์ที่เมื่อนำมาบวกกันแล้วได้เท่ากับ target คุณสามารถสมมติได้ว่าโจทย์แต่ละข้อมีคำตอบเพียงคำตอบเดียวเท่านั้น และไม่สามารถใช้สมาชิกตัวเดิมซ้ำสองครั้งได้ คุณสามารถส่งคำตอบในลำดับใดก็ได้



Example 1:

Input: nums = [2,7,11,15], target = 9

Output: [0,1]

Explanation:

The sum of nums[0] and nums[1] is 2 + 7 = 9, which equals the target. Therefore, the indices [0, 1] are returned.

Example 2:

Input: nums = [3,2,4], target = 6

Output: [1,2]

Explanation:

The sum of nums[1] and nums[2] is 2 + 4 = 6, which equals the target. Therefore, the indices [1, 2] are returned.

Example 3:

Input: nums = [3,3], target = 6

Output: [0,1]

Explanation:

The sum of nums[0] and nums[1] is 3 + 3 = 6, which equals the target. Therefore, the indices [0, 1] are returned.