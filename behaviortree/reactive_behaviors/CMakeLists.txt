cmake_minimum_required(VERSION 3.8)
project(reactive_behaviors)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(behaviortree_cpp REQUIRED)
find_package(ament_index_cpp REQUIRED)

add_executable(reactive_behaviors src/reactive_behaviors.cpp)

ament_target_dependencies(reactive_behaviors behaviortree_cpp ament_index_cpp)

install(TARGETS reactive_behaviors DESTINATION lib/${PROJECT_NAME})
install(FILES src/my_tree.xml DESTINATION share/${PROJECT_NAME})

ament_package()