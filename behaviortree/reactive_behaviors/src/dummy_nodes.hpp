#ifndef DUMMY_NODES_HPP
#define DUMMY_NODES_HPP

#include<iostream>
#include<string>
#include<behaviortree_cpp/behavior_tree.h>
#include<behaviortree_cpp/action_node.h>

struct Pose2D{
    double x, y, theta;
}

namespace chr = std::chrono;

class MoveBaseAction : public BT::AsyncActionNode{
    public:
        MoveBaseAction(const std::string& name, const BT::NodeConfiguration& config) : BT::AsyncActionNode(name, config) {}

        static BT::PortsList providedPorts(){
            return { BT::InputPort<Pose2D>("goal") };
        }

        // 在进入节点时调用
        BT::NodeStatus onStart() override;
        // 如果 onStart() 返回 RUNNING 则持续调用直到非 RUNNING 状态
        BT::NodeStatus onRunning() override;
        // 节点被其它节点打断时调用
        void onHalted() override;

    private:
        Pose2D _goal;
        chr::system_clock::time_point _completion_time;
}

//----------------------------------------------------------

BT::NodeStatus MoveBaseAction::onStart(){
    if(!getInput<Pose2D>("goal", _goal)){
        throw BT::RuntimeError("missing required input [goal]");
    } else {
        printf("[ MoveBase: SEND REQUEST ]. goal: x=%f y=%f theta=%f\n", _goal.x, _goal.y, _goal.theta);
    }

    // 使用计数器来模拟需要一定时间完成的动作（200ms）
    _completion_time = chr::system_clock::now() + chr::milliseconds(200);

    return NodeStatus::RUNNING;
}

BT::NodeStatus MoveBaseAction::onRunning(){
    // 假装检查耗时，勿在检查中阻塞过长时间
    std::this_thread::sleep_for(chr::milliseconds(10));

    // 一段时候后，完成了动作，返回 SUCCESS
    if(chr::system_clock::now() >= _completion_time){
        printf("[ MoveBase: DONE ]\n");
        return NodeStatus::SUCCESS;
    }

    return BT::NodeStatus::RUNNING;
}

void MoveBaseAction::onHalted(){
    printf("[ MoveBase: 被中止 ]");
}

#endif // DUMMY_NODES_HPP