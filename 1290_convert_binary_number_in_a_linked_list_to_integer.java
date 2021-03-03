/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public int getDecimalValue(ListNode head) {
        int val = head.val;
        while (head.next != null) {
            val = val * 2 + head.next.val;
            head = head.next;
        }
        return val;
    }
}
/**
 * Runtime: 0 ms, faster than 100.00% of Java online submissions for Convert Binary Number in a Linked List to Integer.
 * Memory Usage: 36.5 MB, less than 46.84% of Java online submissions for Convert Binary Number in a Linked List to Integer.
 */

