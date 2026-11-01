# 13. Roman to Integer

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

ตัวอย่างเช่น เลข 2 ในระบบเลขโรมันจะเขียนแทนด้วย II ซึ่งก็คือการนำ I สองตัวมาวางรวมกัน เลข 12 เขียนแทนด้วย XII ซึ่งก็คือ X รวมกับ II ส่วนเลข 27 เขียนแทนด้วย XXVII ซึ่งก็คือ XX รวมกับ V และ II

โดยปกติแล้ว เลขโรมันจะเรียงลำดับจากค่ามากไปหาน้อยจากซ้ายไปขวา อย่างไรก็ตาม เลข 4 จะไม่เขียนแทนด้วย IIII แต่จะเขียนแทนด้วย IV เนื่องจากมีการวาง I ไว้หน้า V ซึ่งหมายถึงการนำค่ามาลบกันจนได้เป็น 4 หลักการเดียวกันนี้ยังใช้กับเลข 9 ซึ่งเขียนแทนด้วย IX โดยมีกรณีที่ต้องใช้วิธีการลบอยู่ทั้งหมด 6 รูปแบบ ดังนี้:

- I สามารถวางไว้หน้า V (5) และ X (10) เพื่อแทนค่า 4 และ 9
- X สามารถวางไว้หน้า L (50) และ C (100) เพื่อแทนค่า 40 และ 90
- C สามารถวางไว้หน้า D (500) และ M (1000) เพื่อแทนค่า 400 และ 900

จงแปลงเลขโรมันที่กำหนดให้เป็นจำนวนเต็ม

Example 1:

Input: s = "III"
Output: 3
Explanation: III = 3.

Example 2:

Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.

Example 3:

Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

 

Constraints:

    1 <= s.length <= 15
    s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
    It is guaranteed that s is a valid roman numeral in the range [1, 3999].
