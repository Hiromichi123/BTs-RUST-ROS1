#include <behaviortree_cpp_v3/bt_factory.h>
#include <ament_index_cpp/get_package_share_directory.hpp>
#include <filesystem>
#include "dummy_nodes.hpp"

int main()
{  
    BT::BehaviorTreeFactory factory;
    factory.registerNodeType<SaySomething>("SaySomething");
    factory.registerNodeType<ThinkWhatToSay>("ThinkWhatToSay");

    std::string package_share_path = ament_index_cpp::get_package_share_directory("blackboard_and_ports");
    std::string xml_path = package_share_path + "/my_tree.xml";
    auto tree = factory.createTreeFromFile(xml_path);

    tree.tickRootWhileRunning();
    return 0;
}

/*  预期输出：
  Robot says: hello
  Robot says: The answer is 42
*/