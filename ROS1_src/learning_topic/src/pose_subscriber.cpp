//该例程将订阅/turtle1/pose话题，消息类型geometry_msgs::Twist
#include<ros/ros.h>
#include"turtlesim/Pose.h"

//消息回调函数
void poseCallback(const turtlesim::Pose::ConstPtr& msg)
{
    //打印消息
    ROS_INFO("Turtle pose:x:%0.6f,y:%0.6f",msg->x,msg->y);
}

int main(int argc,char**argv)
{
    ros::init(argc,argv,"pose_subscriber");
    ros::NodeHandle n;

    //创建一个Subscriber，订阅名为/turtlr1/pose点topic，注册回调函数poseCallback
    ros::Subscriber pose_pub=n.subscribe("/turtle1/pose",10,poseCallback);

    ros::spin(); //循环等待回调函数

    return 0;
}