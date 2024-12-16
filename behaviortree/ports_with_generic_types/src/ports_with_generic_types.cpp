#include <behaviortree_cpp/bt_factory.h>
#include <ament_index_cpp/get_package_share_directory.hpp>
#include <filesystem>
#include "dummy_nodes.hpp"
/*static const char* xml_text = R"(

 <root BTCPP_format="4" >
     <BehaviorTree ID="MainTree">
        <Sequence name="root">
            <CalculateGoal goal="{GoalPosition}" />
            <PrintTarget   target="{GoalPosition}" />
            <Script        code=" OtherGoal:='-1;3' " />
            <PrintTarget   target="{OtherGoal}" />
        </Sequence>
     </BehaviorTree>
 </root>
 )";*/

int main()
{
    BT::BehaviorTreeFactory factory;
    factory.registerNodeType<CalculateGoal>("CalculateGoal");
    factory.registerNodeType<PrintTarget>("PrintTarget");

    std::string package_share_path = ament_index_cpp::get_package_share_directory("ports_with_generic_types");
    std::string xml_path = package_share_path + "/my_tree.xml";
    auto tree = factory.createTreeFromFile(xml_path);
    // 另一种直接从程序中读取xml文本的方法
    //auto tree = factory.createTreeFromFile(xml_text);

    tree.tickWhileRunning();

    return 0;
}

/* 预期输出：
*
    Target positions: [ 1.1, 2.3 ]
    Target positions: [ -1.0, 3.0 ]
*/