//该例程将产生tf数据，并计算、发布turtle2点速度指令
#include<ros/ros.h>
#include<tf/transform_broadcaster.h>
#include<turtlesim/Pose.h>

std::string turtle_name;

void poseCallback(const turtlesim::Pose::ConstPtr& msg)
{
    //创建tf广播器实例，静态声明减少每次循环重新创建的开销
    static tf::TransformBroadcaster br;

    //初始化tf数据
    tf::Transform transform; //创建一个tf变换对象
    transform.setOrigin(tf::Vector3(msg->x,msg->y,0.0)); //平移设置变换的起源为turtle的位置
    tf::Quaternion q; //创建一个tf四元数对象
    q.setRPY(0,0,msg->theta); //设置四元数的滚动、俯仰和偏航角，即 turtle 的朝向
    transform.setRotation(q); //设置变换的旋转部分

    //广播world与海龟的坐标系之间的当前时间坐标关系的tf数据
    br.sendTransform(tf::StampedTransform(transform,ros::Time::now(),"world",turtle_name)); //发送变换数据
}

int main(int argc,char**argv)
{
    ros::init(argc,argv,"my_tf_broadcaster");

    //输入参数作为海龟的名字(默认为1)
    if(argc!=2) //如果输入参数数量不正确
    {
        ROS_ERROR("need turtle name as argument"); //打印错误信息
        return -1; //结束程序
    }

    turtle_name=argv[1]; //将命令行参数赋值给turtle_name变量

    //订阅海龟的位姿话题
    ros::NodeHandle node;
    ros::Subscriber sub = node.subscribe(turtle_name+"/pose",10,&poseCallback);

    ros::spin();

    return 0;
}