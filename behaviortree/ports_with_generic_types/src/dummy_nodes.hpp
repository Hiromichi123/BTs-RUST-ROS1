#ifndef DUMMY_NODES_HPP
#define DUMMY_NODES_HPP

#include<iostream>
#include<string>
#include<behaviortree_cpp_v3/behavior_tree.h>
#include<behaviortree_cpp_v3/action_node.h>
using namespace BT;

struct Position2D 
{ 
  double x;
  double y; 
};

// 模版特化，将字符串转换为Position2D，必须显示声明命名空间BT
namespace BT
{
    template <> inline Position2D convertFromString(StringView str){
        auto parts = splitString(str, ';');
        if (parts.size() != 2){
            throw RuntimeError("invalid input");
        } else {
            Position2D output;
            output.x = convertFromString<double>(parts[0]);
            output.y = convertFromString<double>(parts[1]);
            return output;
        }
    }
}

class CalculateGoal: public SyncActionNode
{
    public:
        CalculateGoal(const std::string& name, const NodeConfiguration& config): SyncActionNode(name, config){}

    static PortsList providedPorts(){
        return { OutputPort<Position2D>("goal") };
    }

    NodeStatus tick() override{
        Position2D mygoal = {1.1, 2.3};
        setOutput<Position2D>("goal", mygoal);
        return NodeStatus::SUCCESS;
    }
};

class PrintTarget: public SyncActionNode{
    public:
        PrintTarget(const std::string& name, const NodeConfiguration& config): SyncActionNode(name, config){}

        static PortsList providedPorts(){
            const char* description = "Simply print the goal on console...";
            return { InputPort<Position2D>("target", description) };
        }

        NodeStatus tick() override{
            auto res = getInput<Position2D>("target");
            if (!res){
                throw RuntimeError("error reading port [target]:", res.error());
            }
            Position2D target = res.value();
            printf("Target position: [%.1f, %.1f]\n", target.x, target.y);
            return NodeStatus::SUCCESS;
        }
};

#endif  // DUMMY_NODES_HPP