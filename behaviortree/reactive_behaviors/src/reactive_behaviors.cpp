#include <behaviortree_cpp/bt_factory.h>
#include <ament_index_cpp/get_package_share_directory.hpp>
#include <filesystem>
#include "dummy_nodes.hpp"

int main(){
    BT::BehaviorTreeFactory factory;
    factory.registerSimpleCondition("BatteryOK", std::bind(CheckBattery));
    factory.registerNodeType<MoveBaseAction>("MoveBase");
    factory.registerNodeType<SaySomething>("SaySomething");

    auto tree = factory.createTreeFromFile(ament_index_cpp::get_package_share_directory("reactive_behaviors") + "/my_tree.xml");

    auto status = tree.tickOnce();
    do {std::cout << "--- ticking\n";
        status = tree.tickOnce();
        std::cout << "--- stuatus:" << toStr(status) << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    } while(status == BT::NodeStatus::RUNNING);

    return 0;
}

/* 预期输出：
*
    [ Battery: OK ]
    mission started...
    [ MoveBase: SEND REQUEST ]. goal: x=1.000000 y=2.000000 theta=3.000000
    --- ticking
    --- stuatus:RUNNING
    --- ticking
    --- stuatus:RUNNING
    --- ticking
    [ MoveBase: DONE ]
    mission completed!
    --- stuatus:SUCCESS
*/