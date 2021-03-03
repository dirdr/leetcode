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
    public ListNode middleNode(ListNode head) {
        int total = 1;
        ListNode firstTravel = head;
        while(firstTravel.next != null) {
            firstTravel = firstTravel.next;
            total++;
        }
        firstTravel = head;
        int middle = total/2;
        int i = 0;
        while (i < middle) {
            firstTravel = firstTravel.next;
            i++;
        }
        return firstTravel;
    }
}

/**
 * Runtime: 0 ms, faster than 100.00% of Java online submissions for Middle of the Linked List.
 * Memory Usage: 36.2 MB, less than 78.74% of Java online submissions for Middle of the Linked List.
 */
 
