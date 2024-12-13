#！/usr/bin/env python
#-*-coding:utf-8-*-
import sys
import rospy
from std_srvs.srv import Empty

def parameter_config():
    rospy.init_node('parameter_config',anonymous=True)

    red=rospy.get_param('/turtlesim/background_r')
    green=rospy.get_param('/turtlesim/background_g')
    blue=rospy.get_param('/turtlesim/background_b')
    rospy.loginfo("Get Background color[%d,%d,%d]",red,green,blue)

    rospy.set_param('/turtlesim/background_r',255)
    rospy.set_param('/turtlesim/background_g',255)
    rospy.set_param('/turtlesim/background_b',255)
    rospy.loginfo("Set Background Color[255,255,255]")

    red=rospy.get_param('/turtlesim/background_r')
    green=rospy.get_param('/turtlesin/background_g')
    blue=rospy.get_param('/turtlesin/background_b')
    rospy.loginfo("Get Background color[%d,%d,%d]",red,green,blue)

    #发现/clear服务后，创建一个服务客户端，连接名为/clear的service
    rospy.wait_for_service('/clear')
    try:
        clear_background=rospy.ServiceProxy('/clear',Empty)
        
        #请求服务调用，输入请求数据
        response=clear_background()
        return response
    except rospy.ServiceException,e:
        print "Service Call failed:%s"%e

if __name__=='__main__':
    parameter_config()