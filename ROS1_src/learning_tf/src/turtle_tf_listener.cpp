//该例程将监听tf数据，并计算、发布turtle2点速度指令
#include<ros/ros.h>
#include<tf/transform_listener.h>
#include<geometry_msgs/Twist.h>
#include<turtlesim/Spawn.h>

int main(int argc,char**argv)
{
    ros::init(argc,argv,"my_tf_listener");
    ros::NodeHandle node;

    //请求产生turtle2
    ros::service::waitForService("/spawn");
    ros::ServiceClient add_turtle=node.serviceClient<turtlesim::Spawn>("/spawn");
    turtlesim::Spawn srv;
    add_turtle.call(srv);

    //创建发布turtles速度控制指令的发布者
    ros::Publisher turtle_vel = node.advertise<geometry_msgs::Twist>("/turtle2/cmd_vel",10);

    //创建tf的监视器
    tf::TransformListener listener;

    ros::Rate rate(10.0);
    while(node.ok())
    {
        //获取turtle1与turtle2坐标系之间当前时间的tf数据，查询等待3秒
        tf::StampedTransform transform;
        try
        {
            listener.waitForTransform("/turtle2","/turtle1",ros::Time(0),ros::Duration(3.0));
            listener.lookupTransform("/turtle2","/turtle1",ros::Time(0),transform);
        }
        catch(tf::TransformException &ex)
        {
            ROS_ERROR("%s",ex.what());
            ros::Duration(1.0).sleep();
            continue;
        }

        //根据turtle1与turtle2坐标系的位置关系，发布turtle2的速度控制指令让其向turtle1移动
        geometry_msgs::Twist vel_msg;
        vel_msg.angular.z=4.0*atan2(transform.getOrigin().y(),
                                    transform.getOrigin().x());
        vel_msg.linear.x=0.5*sqrt(pow(transform.getOrigin().x(),2)+
                                  pow(transform.getOrigin().y(),2));
        turtle_vel.publish(vel_msg);
        
        rate.sleep();
    }
}