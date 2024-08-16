# very crude turing machine, currently doing binary addition (although not 100% if it works correctly)
# if tape or transitions is changed make sure to change the length as well

def find_trans():
    i = 0
    while i < trans_len:
        trans = transitions[i]
        if trans[0] == state and trans[1] == current_symbol:
            return [trans[2], trans[3], trans[4]]
        else:
            i = i + 1
    return 1

tape = [1,0,0,0,"_",1,1,1]
tape_len = 8 # no length function
# a transition is of the form:
# [current_state,current_symbol,next_state,write_symbol,direction]
transitions = [["q0","_","q0","_","R"], ["q0",0,"q1",0,"R"], ["q0",1,"q1",1,"R"], ["q1","A","q1","A","R"], ["q1","B","q1","B","R"], ["q1",0,"q1",0,"R"], ["q1",1,"q1",1,"R"], ["q1","_","q2","_","R"], ["q2",0,"q2",0,"R"], ["q2",1,"q2",1,"R"], ["q2","_","q3","_","L"], ["q3",0,"q4","_","L"], ["q3",1,"q6","_","L"], ["q3","_","q9","_","L"], ["q4",0,"q4",0,"L"], ["q4",1,"q4",1,"L"], ["q4","_","q5","_","L"], ["q5","A","q5","A","L"], ["q5","B","q5","B","L"], ["q5",0,"q1","A","R"], ["q5",1,"q1","B","R"], ["q6",0,"q6",0,"L"], ["q6",1,"q6",1,"L"], ["q6","_","q7","_","L"], ["q7","A","q7","A","L"], ["q7","B","q7","B","L"], ["q7",0,"q1","B","R"], ["q7",1,"q8","A","L"], ["q8",1,"q8",0,"L"], ["q8",0,"q1",1,"R"], ["q8","_","q1",1,"R"], ["q9","A","q9",0,"L"], ["q9","B","q9",1,"L"], ["q9",0,"q_accept",0,"R"], ["q9",1,"q_accept",1,"R"], ["q9","_","q_accept","_","R"]]
trans_len = 36 # again no length function I wish I made one
initial_state = "q0"
accept_state = "q_accept"
reject_state = "q_reject"
head_pos = 0
state = initial_state
max_steps = 1000

steps = 0
while state != accept_state and state != reject_state:
    if steps > max_steps:
        print("Maximum steps exceeded")
        return

    # figure out current symbol
    if head_pos < tape_len and head_pos >= 0:
        current_symbol = tape[head_pos]
    else:
        current_symbol = "_"

    # find transition and return either error if not found or new information
    step_res = find_trans()
    if step_res == 1:
        print("No valid transition found")
        return
    else:
        new_state = step_res[0]
        new_symbol = step_res[1]
        direction = step_res[2]

    # update tape
    if head_pos < tape_len and head_pos >= 0:
        tape[head_pos] = new_symbol
    else:
        if head_pos >= 0:
            tape = tape + [new_symbol]
            tape_len = tape_len + 1
        else:
            tape = [new_symbol] + tape
            tape_len = tape_len + 1
            head_pos = 0

    # update state
    state = new_state

    # move
    if direction == "R":
        head_pos = head_pos + 1
    if direction == "L":
        head_pos = head_pos - 1
        
    steps = steps + 1

if state == accept_state:
    print(tape)
    print("Accepted!")
else:
    print("Rejected!")
