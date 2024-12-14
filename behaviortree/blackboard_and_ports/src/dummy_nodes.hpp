#ifndef DUMMY_NODES_HPP
#define DUMMY_NODES_HPP

#include<iostream>
#include<string>
#include<optional>
#include<behaviortree_cpp_v3/behavior_tree.h>
#include<behaviortree_cpp_v3/action_node.h>  // ActionNodeBase 和 SyncActionNode

using namespace BT;

// 输入端口Input Ports
class SaySomething : public SyncActionNode{
    public:
        SaySomething(const std::string& name, const BT::NodeConfiguration& config) : SyncActionNode(name, config){}

    // 必须实现的静态方法
    static PortsList providedPorts(){
        // 表示该节点有名为message的输入端口
        return { InputPort<std::string>("message") };
    }

    NodeStatus tick() override{
        // 获取消息抛出错误的写法
        BT::Optional<std::string> msg = getInput<std::string>("message");
        if (!msg)
        {
            throw BT::RuntimeError("missing required input [message]");
        }
        // 使用 value() 提取消息
        std::cout<<msg.value()<<std::endl;
        return NodeStatus::SUCCESS;
    }
};

// 输出端口Output Ports
class ThinkWhatToSay : public SyncActionNode{
    public:
        ThinkWhatToSay(const std::string& name, const BT::NodeConfiguration& config) : SyncActionNode(name, config){}

    static PortsList providedPorts(){
        // 表示该节点有名为text的输出端口
        return { OutputPort<std::string>("text") };
    }

    NodeStatus tick() override{
        // 每次tick都输出42
        setOutput("text", "The answer is 42");
        return NodeStatus::SUCCESS;
    }
};

#endif  // DUMMY_NODES_HPP
