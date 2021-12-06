use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        
        // Dont need to find an open spot as it will always be the next index
        self.push(ele); // Add the new element to the heap
        let mut curr_idx = self.len() - 1;

        // Now we need to sift up into the correct position
        // Just swap with the parent if smaller, dont worry about siblings
        while curr_idx != 0 {
            if self[(curr_idx-1)/2] > self[curr_idx] { // If the parent is greater than the current (ele)
                self.swap((curr_idx-1)/2, curr_idx); // Swap their positions
                curr_idx = (curr_idx-1)/2; // And set the new current index to where the parent was
            } else {
                curr_idx = 0; // Otherwise stop, ele is in the right place
            }
        }
        
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        
        let mut curr_idx = 0;

        // If there is no root return None
        if self.len() == 0 {
            return None;
        }

        // Otherwise, remove the root and replace it with the last element
        let root = self.swap_remove(0);

        // Now we need to sift the new root element down to its correct place to restore the heap
        // To sift down we check to find the smallest child and swap if the current element is larger
        
        while curr_idx < self.len() { // Stop when you reach the end of the heap
            if (curr_idx*2)+1 < self.len() { // If there is a left child 
                if (curr_idx*2)+2 < self.len() { // If there is a right child
                    
                    // If we get here then both children exist so find the smaller one
                    if self[(curr_idx*2)+1] < self[(curr_idx*2)+2] && self[(curr_idx*2)+1] < self[curr_idx] {
                        // If the left child is smaller and smaller than the current element swap them
                        self.swap((curr_idx*2)+1, curr_idx);
                        curr_idx = (curr_idx*2)+1; // Set the current index to where the child was and go again
                    } else if self[(curr_idx*2)+2] < self[(curr_idx*2)+1] && self[(curr_idx*2)+2] < self[curr_idx] {
                        // If the left child is smaller and smaller than the current element swap them
                        self.swap((curr_idx*2)+2, curr_idx);
                        curr_idx = (curr_idx*2)+2; // Set the current index to where the child was and go again
                    } else {
                        // If neither child is smaller than the current element it is in the right place so dont swap
                        curr_idx = self.len();
                    }

        
                } else { // If there is no right child but there is a left child
                    if self[(curr_idx*2)+1] < self[curr_idx] {
                        // If the left child is smaller than the current element swap them
                        self.swap((curr_idx*2)+1, curr_idx);
                        curr_idx = (curr_idx*2)+1; // Set the current index to where the child was and go again
                    } else {
                        // Otherwise, dont swap and stop
                        curr_idx = self.len();
                    }
                }
            } else { // If there is no left child
                //  Not having a left child means that there is no right child so end
                curr_idx = self.len();
            }
        }
        // Return the root
        return Some (root);
    }
        
    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        return self.get(0);
    }
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    
    // Plan:
    // For each ally, check the distance to each enemy and add that pairing to the priority heap
    // Then dequeue Node pairings one at a time until a stark enemy that has not been seen gets popped off

    let mut pairs = Vec::new();
    let mut taken = Vec::new();

    // For each ally enemy pair, add a node prioritized by distance to the heap
    for (ally, ally_pos) in allies {
        for (enemy, enemy_pos) in enemies {
            let pair = Node {
                priority: distance(*ally_pos, *enemy_pos),
                data: (ally, enemy, enemy_pos)
            };
            pairs.enqueue(pair);
        }
    }
    
    while pairs.peek() != None {
        let closest_pair = pairs.dequeue();
        match closest_pair {
            None => return ("Something Very Bad Happened with the dequeuing of pairs", 0, 0),
            Some(pair_node) => {
                if pair_node.data.0 == &"Stark" {
                    if taken.contains(pair_node.data.1) == false {
                        return (pair_node.data.1, pair_node.data.2.0, pair_node.data.2.1);
                    }
                } else if taken.contains(pair_node.data.0) == false && taken.contains(pair_node.data.1) == false {
                    taken.push(pair_node.data.0);
                    taken.push(pair_node.data.1);
                } else {
                    ; // Do nothing if one member of the pair is already taken
                }
            }
        }
        
    }

    return ("Something Very Bad Happened Because No Result was Found", 0, 0);

}

