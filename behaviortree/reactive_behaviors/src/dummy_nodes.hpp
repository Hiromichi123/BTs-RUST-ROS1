#ifndef DUMMY_NODES_HPP
#define DUMMY_NODES_HPP

#include<iostream>
#include<string>
#include<behaviortree_cpp/behavior_tree.h>
#include<behaviortree_cpp/action_node.h>
using namespace BT;
namespace chr = std::chrono;

struct Pose2D{
    double x, y, theta;
};

namespace BT{
    template <> inline Pose2D convertFromString(StringView str){
        auto parts = splitString(str, ';');
        if (parts.size() != 3){
            throw RuntimeError("invalid input");
        } else {
            Pose2D output;
            output.x = convertFromString<double>(parts[0]);
            output.y = convertFromString<double>(parts[1]);
            output.theta = convertFromString<double>(parts[2]);
            return output;
        }
    }
}

class MoveBaseAction : public StatefulActionNode{
    public:
        MoveBaseAction(const std::string& name, const BT::NodeConfiguration& config) : BT::StatefulActionNode(name, config) {}

        static BT::PortsList providedPorts(){
            return { BT::InputPort<Pose2D>("goal") };
        }

        // 在进入节点时调用
        BT::NodeStatus onStart() override{
            if(!getInput<Pose2D>("goal", _goal)){
                throw BT::RuntimeError("missing required input [goal]");
            } else {
                printf("[ MoveBase: SEND REQUEST ]. goal: x=%f y=%f theta=%f\n", _goal.x, _goal.y, _goal.theta);
            }

            // 使用计数器来模拟需要一定时间完成的动作（200ms）
            _completion_time = chr::system_clock::now() + chr::milliseconds(200);

            return NodeStatus::RUNNING;
        }

        // 如果 onStart() 返回 RUNNING 则持续调用直到非 RUNNING 状态
        BT::NodeStatus onRunning() override{
            // 假装检查耗时，勿在检查中阻塞过长时间
            std::this_thread::sleep_for(chr::milliseconds(10));

            // 一段时候后，完成了动作，返回 SUCCESS
            if(chr::system_clock::now() >= _completion_time){
                printf("[ MoveBase: DONE ]\n");
                return NodeStatus::SUCCESS;
            }

            return BT::NodeStatus::RUNNING;
        };


        // 节点被其它节点打断时调用
        void onHalted() override{
            printf("[ MoveBase: 被中止 ]");
        };

    private:
        Pose2D _goal;
        chr::system_clock::time_point _completion_time;
};

//---从之前的例程借用-------------------------------------------------------
BT::NodeStatus CheckBattery()
{
  std::cout << "[ Battery: OK ]" << std::endl;
  return BT::NodeStatus::SUCCESS;
}

class SaySomething : public SyncActionNode{
    public:
        SaySomething(const std::string& name, const BT::NodeConfiguration& config) : SyncActionNode(name, config){}

    static PortsList providedPorts(){
        return { InputPort<std::string>("message") };
    }

    NodeStatus tick() override{
        BT::Expected<std::string> msg = getInput<std::string>("message");
        if (!msg)
        {
            throw BT::RuntimeError("missing required input [message]");
        }
        std::cout<<msg.value()<<std::endl;
        return NodeStatus::SUCCESS;
    }
};

#endif // DUMMY_NODES_HPP