//该历程将发布/person_info话题，自定义消息类型lesrning_topic::Person
#include<ros/ros.h>
#include"learning_topic/Person.h"

int main(int argc,char**argv)
{
    ros::init(argc,argv,"person_publisher");
    ros::NodeHandle n;

    //创建一个Publisher，发布名为/person_info的topic，消息类型为learning_topic::Person，队列长度为10
    ros::Publisher person_info_pub=n.advertise<learning_topic::Person>("/person_info",10);

    ros::Rate loop_rate(1);
    
    int count=0;
    while(ros::ok())
    {
        //chushihualearning_topic::Person类型的消息
        learning_topic::Person person_msg;
        person_msg.name="Tom";
        person_msg.age=18;
        person_msg.sex=learning_topic::Person::male;

        person_info_pub.publish(person_msg);

        ROS_INFO("Publish Person Info:name:%s age:%d sex:%d"
                 ,person_msg.name.c_str(),person_msg.age,person_msg.sex);

        loop_rate.sleep();
    }
    
    return 0;
}