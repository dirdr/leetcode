/**
 * Definition for singly-linked list.
 * class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public boolean hasCycle(ListNode head) {
        ListNode current = head;
        HashSet<ListNode> set = new HashSet<>();
        while (current != null) {
            if (set.contains(current)) {
                return true;
            } else {
                set.add(current);
            }
            current = current.next;
        }
        return false;
    }
}

/*
* Runtime: 6 ms, faster than 14.85% of Java online submissions for Linked List Cycle.
* Memory Usage: 48.4 MB, less than 5.04% of Java online submissions for Linked List Cycle.
*/
