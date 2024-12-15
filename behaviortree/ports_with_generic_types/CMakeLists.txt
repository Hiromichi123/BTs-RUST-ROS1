cmake_minimum_required(VERSION 3.8)
project(ports_with_generic_types)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(behaviortree_cpp_v3 REQUIRED)
find_package(ament_index_cpp REQUIRED)

add_executable(ports_with_generic_types src/ports_with_generic_types.cpp)

ament_target_dependencies(ports_with_generic_types behaviortree_cpp_v3 ament_index_cpp)

install(TARGETS ports_with_generic_types DESTINATION lib/${PROJECT_NAME})
install(FILES src/my_tree.xml DESTINATION share/${PROJECT_NAME})

ament_package()