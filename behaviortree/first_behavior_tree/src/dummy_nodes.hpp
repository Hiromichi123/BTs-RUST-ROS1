#ifndef DUMMY_NODES_HPP
#define DUMMY_NODES_HPP

#include<iostream>
#include<behaviortree_cpp_v3/behavior_tree.h>

// 自定义同步动作节点
class ApproachObject : public BT::ActionNodeBase
{
    public:
        ApproachObject(const std::string& name) : BT::ActionNodeBase(name, {}) {}

        // tick()回调函数，必须重写
        BT::NodeStatus tick() override
        {
            std::cout << "ApproachObject:" << this->name() << std::endl;
            return BT::NodeStatus::SUCCESS;
        }

        // 当节点被终止时调用
        void halt() override
        {
            std::cout << "Halting ApproachObject: " << this->name() << std::endl;
        }
};

// 检查电池状态的函数
BT::NodeStatus CheckBattery()
{
  std::cout << "[ Battery: OK ]" << std::endl;
  return BT::NodeStatus::SUCCESS;
}

// 夹持器接口类，用于打开和关闭夹持器
class GripperInterface
{
public:
  GripperInterface(): _open(true) {}

  BT::NodeStatus open() 
  {
    _open = true;
    std::cout << "GripperInterface::open" << std::endl;
    return BT::NodeStatus::SUCCESS;
  }

  BT::NodeStatus close() 
  {
    std::cout << "GripperInterface::close" << std::endl;
    _open = false;
    return BT::NodeStatus::SUCCESS;
  }

private:
  bool _open; // 夹持器的状态
};

#endif  // DUMMY_NODES_HPP