cmake_minimum_required(VERSION 3.8)
project(first_behavior_tree)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(behaviortree_cpp_v3 REQUIRED)
find_package(ament_index_cpp REQUIRED)

add_executable(first_behavior_tree src/first_behavior_tree.cpp)

ament_target_dependencies(first_behavior_tree behaviortree_cpp_v3 ament_index_cpp)

install(TARGETS first_behavior_tree DESTINATION lib/${PROJECT_NAME})
install(FILES src/my_tree.xml DESTINATION share/${PROJECT_NAME})

ament_package()