#include <behaviortree_cpp_v3/bt_factory.h>
#include <ament_index_cpp/get_package_share_directory.hpp>
#include <filesystem>
#include "dummy_nodes.hpp"

int main(){
    // 使用 BehaviorTreeFactory 注册自定义节点
    BT::BehaviorTreeFactory factory;

    // 通过继承 ActionNodeBase 创建节点
    factory.registerNodeType<ApproachObject>("ApproachObject");

    // 通过函数指针注册 SimpleCodition 节点
    factory.registerSimpleCondition("CheckBattery", [&](BT::TreeNode &){return CheckBattery();});

    // 使用类方法注册 SimpleActionNode
    GripperInterface gripper;

    factory.registerSimpleAction("OpenGripper", [&](BT::TreeNode &){return gripper.open();});
    factory.registerSimpleAction("CloseGripper", [&](BT::TreeNode &){return gripper.close();});

    // 获取包的共享目录路径
    std::string package_share_path = ament_index_cpp::get_package_share_directory("first_behavior_tree");
    std::string xml_path = package_share_path + "/my_tree.xml";
    // 在运行时动态创建行为树（只在程序启动时启动一次）
    auto tree = factory.createTreeFromFile(xml_path);

    // 执行行为树，tick 动作会根据树的逻辑传递到子节点
    // 注意一下4.6版本接口改为 tickWhileRunning()
    tree.tickRootWhileRunning();

    return 0;
}

/* 预期输出：
*
  [ Battery: OK ]
  GripperInterface::open
  ApproachObject: approach_object
  GripperInterface::close
*/