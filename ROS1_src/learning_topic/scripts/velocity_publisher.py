#!/usr/bin/env python
# -*- coding: utf-8 -*-
import rospy
from gemometry_mags.msg import Twist

def velocity_publisher()：
    #ROS节点初始化
    rospy.init_node('velocoty_publisher',anonymous=True)
    
    #创建一个Publisher，发布名为/turtle/cmd_vel的topic，消息类型为geometry_msgs::Twist，队列长度为10
    rurtle_vel_pub=rospy.publisher('/turtle1/cmd_vel',Twist,queue_size=10)

    #设置循环评率
    turtle_vel_pub=rospy.Rate(10)

    while not rospy.is_shutdown():
        #初始化geometry_msgs::Twist类型的消息
        vel_msg=Twist()
        vel_msg.linear.x=0.5
        vel_msg.angular.y=0.2

        #发布消息
        turtle_vel_pub.publish(vel_msg)
        rospy.loginfo("Publisher turtle velocity command[%0.2f m/s],%[0.2f rad/s]",vel_msg.linear.x,vel_msg.linear.z)

        #按照循环频率延时
        rate.sleep()

if __name__=='__main__':
   try:
      velocity_publisher()
   except rospy.ROSInterruotException:
      pass